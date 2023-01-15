use super::parser_helpers::read_u32_leb128;
use nom::error::ParseError as NomParseError;
use nom::{bytes::complete::take, IResult as NomResult, Slice};

pub type Byte = u8;

#[derive(Debug, PartialEq)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

impl ValType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], ValType> {
        let encode_byte_parsed = take(1usize)(bytes)?;

        let val = match encode_byte_parsed.1[0] {
            NumType::ENCODE_BYTE_I32 => ValType::NumType(NumType::I32),
            NumType::ENCODE_BYTE_I64 => ValType::NumType(NumType::I64),
            NumType::ENCODE_BYTE_F32 => ValType::NumType(NumType::F32),
            NumType::ENCODE_BYTE_F64 => ValType::NumType(NumType::F64),
            VecType::ENCODE_BYTE => ValType::VecType(VecType::V128),
            RefType::ENCODE_BYTE_FUNC_REF => ValType::RefType(RefType::FuncRef),
            RefType::ENCODE_BYTE_EXTERN_REF => ValType::RefType(RefType::ExternRef),
            _ => {
                return Err(nom::Err::Failure(nom::error::Error::new(
                    bytes,
                    nom::error::ErrorKind::Fail,
                )))
            }
        };

        Ok((encode_byte_parsed.0, val))
    }
}

#[derive(Debug, PartialEq)]
pub enum VecType {
    V128,
}

impl VecType {
    pub const ENCODE_BYTE: Byte = 0x7B;
}

#[derive(Debug, PartialEq)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl NumType {
    pub const ENCODE_BYTE_I32: Byte = 0x7F;
    pub const ENCODE_BYTE_I64: Byte = 0x7E;
    pub const ENCODE_BYTE_F32: Byte = 0x7D;
    pub const ENCODE_BYTE_F64: Byte = 0x7C;
}

#[derive(Debug, PartialEq)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

impl RefType {
    pub const ENCODE_BYTE_FUNC_REF: Byte = 0x70;
    pub const ENCODE_BYTE_EXTERN_REF: Byte = 0x6F;

    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, encode_byte_slice) = take(1usize)(bytes)?;

        match encode_byte_slice[0] {
            Self::ENCODE_BYTE_EXTERN_REF => Ok((bytes, RefType::ExternRef)),
            Self::ENCODE_BYTE_FUNC_REF => Ok((bytes, RefType::FuncRef)),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FuncType {
    pub parameters: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
    pub const ENCODE_BYTE: Byte = 0x60;
}

#[derive(Debug)]
pub struct Vector<T: std::fmt::Debug> {
    pub length: U32Type,
    pub elements: Vec<T>,
}

#[derive(Debug)]
pub struct NameType(pub String);

impl NameType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], NameType> {
        match U32Type::parse(bytes).and_then(|(bytes, size)| take(size.0)(bytes)) {
            Ok((bytes, name_bytes)) => std::str::from_utf8(name_bytes)
                .map(|name| (bytes, NameType(name.into())))
                .map_err(|_| {
                    nom::Err::Failure(nom::error::Error::new(bytes, nom::error::ErrorKind::Char))
                }),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug)]
pub struct TableType {
    pub limits: LimitsType,
    pub element_ref_type: RefType,
}

impl TableType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], TableType> {
        let (bytes, element_ref_type) = RefType::parse(bytes)?;
        let (bytes, limits) = LimitsType::parse(bytes)?;

        Ok((
            bytes,
            TableType {
                limits,
                element_ref_type,
            },
        ))
    }
}

#[derive(Debug)]
pub struct LimitsType {
    pub min: U32Type,
    pub max: Option<U32Type>,
}

impl LimitsType {
    const ENCODE_BYTE_MAX_NOT_PRESENT: Byte = 0x00;
    const ENCODE_BYTE_MAX_PRESENT: Byte = 0x01;

    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, has_max_limit_byte_slice) = take(1usize)(bytes)?;

        match has_max_limit_byte_slice[0] {
            Self::ENCODE_BYTE_MAX_NOT_PRESENT => {
                let (bytes, min) = U32Type::parse(bytes)?;

                Ok((bytes, LimitsType { min, max: None }))
            }
            Self::ENCODE_BYTE_MAX_PRESENT => {
                let (bytes, min) = U32Type::parse(bytes)?;
                let (bytes, max) = U32Type::parse(bytes)?;

                Ok((
                    bytes,
                    LimitsType {
                        min,
                        max: Some(max),
                    },
                ))
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug)]
pub struct MemType {
    pub limits: LimitsType,
}

impl MemType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, limits) = LimitsType::parse(bytes)?;

        Ok((bytes, MemType { limits }))
    }
}

#[derive(Debug)]
pub struct GlobalType {
    pub mut_type: MutType,
    pub val_type: ValType,
}

impl GlobalType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, val_type) = ValType::parse(bytes)?;
        let (bytes, mut_type) = MutType::parse(bytes)?;

        Ok((bytes, GlobalType { mut_type, val_type }))
    }
}

#[derive(Debug)]
pub enum MutType {
    Const,
    Var,
}

impl MutType {
    const ENCODE_BYTE_CONST: Byte = 0x00;
    const ENCODE_BYTE_VAR: Byte = 0x01;

    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, mut_type_byte_slice) = take(1usize)(bytes)?;

        match mut_type_byte_slice[0] {
            Self::ENCODE_BYTE_CONST => Ok((bytes, MutType::Const)),
            Self::ENCODE_BYTE_VAR => Ok((bytes, MutType::Var)),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}
#[derive(Debug)]
pub struct TypeIdx(pub U32Type);

#[derive(Debug)]
pub struct FuncIdx(pub U32Type);

#[derive(Debug)]
pub struct TableIdx(pub U32Type);

#[derive(Debug)]
pub struct MemIdx(pub U32Type);

#[derive(Debug)]
pub struct GlobalIdx(pub U32Type);

#[derive(Debug)]
pub struct ElemIdx(pub U32Type);

#[derive(Debug)]
pub struct DataIdx(pub U32Type);

#[derive(Debug)]
pub struct LocalIdx(pub U32Type);

#[derive(Debug)]
pub struct LabelIdx(U32Type);

#[derive(Debug)]
pub struct LaneIdx(Byte);

#[derive(Debug)]
pub struct U32Type(pub u32);

impl U32Type {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], U32Type> {
        let mut pos = 0usize;
        let val = read_u32_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), U32Type(val)))
    }
}

#[derive(Debug)]
pub struct S33Type(pub i64);

#[derive(Debug)]
pub struct I32Type(pub i32);

#[derive(Debug)]
pub struct I64Type(pub i64);

#[derive(Debug)]
pub struct F32Type(pub f32);

#[derive(Debug)]
pub struct F64Type(pub f64);

pub type ParseResult<T> = Result<T, SyntaxError>;

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedSectionIdValue,
    ModuleMagicNotFound,
    ModuleVersionNotFound,
    InvalidModuleSection,
    InvalidTypesModuleSection,
    InvalidCodeModuleSection,
    InvalidFuncsModuleSection,
    InvalidImportsModuleSection,
    InvalidTablesModuleSection,
    InvalidMemsModuleSection,
    InvalidGlobalsModuleSection,
    InvalidStartModuleSection,
    InvalidElementSegmentModuleSection,
    UnexpectedModuleSectionId,
}

pub struct NomError;

impl NomParseError<NomError> for NomError {
    fn from_error_kind(input: NomError, kind: nom::error::ErrorKind) -> Self {
        NomError
    }

    fn append(input: NomError, kind: nom::error::ErrorKind, other: Self) -> Self {
        NomError
    }
}
