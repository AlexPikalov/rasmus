use super::binary::parse_trait::ParseWithNom;
use super::binary::parser_helpers::{
    parse as nom_parse, read_s33_leb128, read_u32_leb128, read_u64_leb128,
};
use super::instructions::ExpressionType;
use nom::error::ParseError as NomParseError;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_f64},
    IResult as NomResult, Slice,
};

pub type Byte = u8;

#[derive(Debug, PartialEq, Clone)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

impl ValType {
    pub fn recognize(byte: Byte) -> Option<Self> {
        match byte {
            NumType::ENCODE_BYTE_I32 => Some(ValType::NumType(NumType::I32)),
            NumType::ENCODE_BYTE_I64 => Some(ValType::NumType(NumType::I64)),
            NumType::ENCODE_BYTE_F32 => Some(ValType::NumType(NumType::F32)),
            NumType::ENCODE_BYTE_F64 => Some(ValType::NumType(NumType::F64)),
            VecType::ENCODE_BYTE => Some(ValType::VecType(VecType::V128)),
            RefType::ENCODE_BYTE_FUNC_REF => Some(ValType::RefType(RefType::FuncRef)),
            RefType::ENCODE_BYTE_EXTERN_REF => Some(ValType::RefType(RefType::ExternRef)),
            _ => None,
        }
    }

    pub fn get_num_types() -> Vec<ValType> {
        let all_num_types = NumType::get_all();
        let mut num_types = Vec::with_capacity(all_num_types.len());

        for num_type in all_num_types {
            num_types.push(Self::NumType(num_type))
        }

        num_types
    }

    pub fn get_ref_types() -> Vec<ValType> {
        let all_ref_types = RefType::get_all();
        let mut ref_types = Vec::with_capacity(all_ref_types.len());

        for ref_type in all_ref_types {
            ref_types.push(Self::RefType(ref_type))
        }

        ref_types
    }

    pub fn v128() -> Self {
        Self::VecType(VecType::V128)
    }

    pub fn i32() -> Self {
        Self::NumType(NumType::I32)
    }

    pub fn i64() -> Self {
        Self::NumType(NumType::I64)
    }

    pub fn f32() -> Self {
        Self::NumType(NumType::F32)
    }

    pub fn f64() -> Self {
        Self::NumType(NumType::F64)
    }
}

impl ParseWithNom for ValType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], ValType> {
        let encode_byte_parsed = take(1usize)(bytes)?;

        Self::recognize(encode_byte_parsed.1[0])
            .ok_or_else(|| {
                nom::Err::Failure(nom::error::Error::new(bytes, nom::error::ErrorKind::Fail))
            })
            .map(|val| (encode_byte_parsed.0, val))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ResultType(pub Vec<ValType>);

impl ParseWithNom for ResultType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        nom_parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VecType {
    V128,
}

impl VecType {
    pub const ENCODE_BYTE: Byte = 0x7B;
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn get_all() -> Vec<Self> {
        vec![Self::I32, Self::I64, Self::F32, Self::F64]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

impl RefType {
    pub const ENCODE_BYTE_FUNC_REF: Byte = 0x70;
    pub const ENCODE_BYTE_EXTERN_REF: Byte = 0x6F;

    pub fn get_all() -> Vec<Self> {
        vec![RefType::FuncRef, RefType::ExternRef]
    }
}

impl ParseWithNom for RefType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct MemType {
    pub limits: LimitsType,
}

impl MemType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, limits) = LimitsType::parse(bytes)?;

        Ok((bytes, MemType { limits }))
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
pub struct TypeIdx(pub U32Type);

impl ParseWithNom for TypeIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncIdx(pub U32Type);

impl ParseWithNom for FuncIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableIdx(pub U32Type);

impl ParseWithNom for TableIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemIdx(pub U32Type);

impl ParseWithNom for MemIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalIdx(pub U32Type);

impl ParseWithNom for GlobalIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElemIdx(pub U32Type);

impl ParseWithNom for ElemIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataIdx(pub U32Type);

impl ParseWithNom for DataIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalIdx(pub U32Type);

impl ParseWithNom for LocalIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabelIdx(U32Type);

impl ParseWithNom for LabelIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        U32Type::parse(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LaneIdx(pub Byte);

impl ParseWithNom for LaneIdx {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        take(1usize)(bytes).map(|(b, v)| (b, Self(v[0])))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct U32Type(pub u32);

impl ParseWithNom for U32Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], U32Type> {
        let mut pos = 0usize;
        let val = read_u32_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), U32Type(val)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct S33Type(pub i64);

impl ParseWithNom for S33Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut pos = 0usize;
        let val = read_s33_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), S33Type(val)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I32Type(pub u32);

impl ParseWithNom for I32Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut pos = 0usize;
        let val = read_u32_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), Self(val)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct I64Type(pub u64);

impl ParseWithNom for I64Type {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let mut pos = 0usize;
        let val = read_u64_leb128(bytes, &mut pos);
        Ok((bytes.slice(pos..), Self(val)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct F32Type(pub f32);

impl ParseWithNom for F32Type {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        le_f32(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct F64Type(pub f64);

impl ParseWithNom for F64Type {
    fn parse(bytes: &[u8]) -> NomResult<&[Byte], Self>
    where
        Self: Sized,
    {
        le_f64(bytes).map(|(b, v)| (b, Self(v)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub func_type: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: ExpressionType,
}

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
    InvalidDatasModuleSection,
    InvalidDataCountModuleSection,
    InvalidVectorLen,
    UnexpectedModuleSectionId,
    DataCountDoesntMatchDataLen,
}

pub struct NomError;

impl NomParseError<NomError> for NomError {
    fn from_error_kind(_input: NomError, _kind: nom::error::ErrorKind) -> Self {
        NomError
    }

    fn append(_input: NomError, _kind: nom::error::ErrorKind, _other: Self) -> Self {
        NomError
    }
}
