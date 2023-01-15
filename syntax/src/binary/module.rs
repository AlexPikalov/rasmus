pub use super::instructions::*;
use super::types::*;

use nom::{bytes::complete::take, IResult as NomResult};

pub struct Module {
    pub types: Vec<FuncType>,
    pub imports: Vec<ImportType>,
    // TODO: rewrite to func
    pub funcs: Vec<TypeIdx>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<GlobalType>,
    pub exports: Vec<ExportType>,
    pub start: Option<StartType>,
    pub elems: Vec<ElementSegmentType>,
    pub code: Vec<CodeType>,
    pub datas: Vec<DataType>,
}

impl Module {
    pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
}

impl Default for Module {
    fn default() -> Self {
        Module {
            types: vec![],
            imports: vec![],
            funcs: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            exports: vec![],
            start: None,
            elems: vec![],
            code: vec![],
            datas: vec![],
        }
    }
}

pub type SectionIdValue = Byte;

pub struct Section<T> {
    pub section_id: SectionId,
    pub size: U32Type,
    pub cont: T,
}

#[derive(Debug, PartialEq)]
pub enum SectionId {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
}

impl SectionId {
    const CUSTOM_SECTION_ID_VALUE: u8 = 0;
    const TYPE_SECTION_ID_VALUE: u8 = 1;
    const IMPORT_SECTION_ID_VALUE: u8 = 2;
    const FUNCTION_SECTION_ID_VALUE: u8 = 3;
    const TABLE_SECTION_ID_VALUE: u8 = 4;
    const MEMORY_SECTION_ID_VALUE: u8 = 5;
    const GLOBAL_SECTION_ID_VALUE: u8 = 6;
    const EXPORT_SECTION_ID_VALUE: u8 = 7;
    const START_SECTION_ID_VALUE: u8 = 8;
    const ELEMENT_SECTION_ID_VALUE: u8 = 9;
    const CODE_SECTION_ID_VALUE: u8 = 10;
    const DATA_SECTION_ID_VALUE: u8 = 11;
    const DATA_COUNT_SECTION_ID_VALUE: u8 = 12;
}

impl TryFrom<SectionIdValue> for SectionId {
    type Error = SyntaxError;

    fn try_from(byte: SectionIdValue) -> ParseResult<Self> {
        match byte {
            Self::CUSTOM_SECTION_ID_VALUE => Ok(SectionId::Custom),
            Self::TYPE_SECTION_ID_VALUE => Ok(SectionId::Type),
            Self::IMPORT_SECTION_ID_VALUE => Ok(SectionId::Import),
            Self::FUNCTION_SECTION_ID_VALUE => Ok(SectionId::Function),
            Self::TABLE_SECTION_ID_VALUE => Ok(SectionId::Table),
            Self::MEMORY_SECTION_ID_VALUE => Ok(SectionId::Memory),
            Self::GLOBAL_SECTION_ID_VALUE => Ok(SectionId::Global),
            Self::EXPORT_SECTION_ID_VALUE => Ok(SectionId::Export),
            Self::START_SECTION_ID_VALUE => Ok(SectionId::Start),
            Self::ELEMENT_SECTION_ID_VALUE => Ok(SectionId::Element),
            Self::CODE_SECTION_ID_VALUE => Ok(SectionId::Code),
            Self::DATA_SECTION_ID_VALUE => Ok(SectionId::Data),
            Self::DATA_COUNT_SECTION_ID_VALUE => Ok(SectionId::DataCount),
            _ => Err(SyntaxError::UnexpectedSectionIdValue),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CustomSection {
    pub name: String,
    pub bytes: Vec<Byte>,
}

#[derive(Debug)]
pub struct ImportType {
    pub module: NameType,
    pub name: NameType,
    pub desc: ImportDescription,
}

#[derive(Debug)]
pub enum ImportDescription {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

impl ImportDescription {
    pub const ENCODE_BYTE_FUNC: Byte = 0x00;
    pub const ENCODE_BYTE_TABLE: Byte = 0x01;
    pub const ENCODE_BYTE_MEM: Byte = 0x02;
    pub const ENCODE_BYTE_GLOBAL: Byte = 0x03;
}

pub struct ExportType {
    pub name: NameType,
    pub desc: ExportDescription,
}

pub enum ExportDescription {
    Func(TypeIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
}

impl ExportDescription {
    pub const ENCODE_BYTE_FUNC: Byte = 0x00;
    pub const ENCODE_BYTE_TABLE: Byte = 0x01;
    pub const ENCODE_BYTE_MEM: Byte = 0x02;
    pub const ENCODE_BYTE_GLOBAL: Byte = 0x03;
}

pub struct StartType {
    pub func: FuncIdx,
}

impl StartType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        U32Type::parse(bytes).map(|(b, val)| (b, StartType { func: FuncIdx(val) }))
    }
}

pub enum ElementSegmentType {
    Active0Functions(Active0FunctionsElementSegmentType),
    ElemKindPassiveFunctions(ElemKindPassiveFunctionsElementSegmentType),
    ElemKindActiveFunctions(ElemKindActiveFunctionsElementSegmentType),
    ElemKindDeclarativeFunctions(ElemKindDeclarativeFunctionsElementSegmentType),
    Active0Expr(Active0ExprElementSegmentType),
    PassiveRef(PassiveRefElementSegmentType),
    ActiveRef(ActiveRefElementSegmentType),
    DeclarativeRef(DeclarativeRefElementSegmentType),
}

impl ElementSegmentType {
    const BITFIELD_ACTIVE0: U32Type = U32Type(0);
    const BITFIELD_ELEM_KIND_PASSIVE: U32Type = U32Type(1);
    const BITFIELD_ELEM_KIND_ACTIVE: U32Type = U32Type(2);
    const BITFIELD_ELEM_KIND_DECLARATIVE: U32Type = U32Type(3);
    const BITFIELD_ACTIVE0_EXPR: U32Type = U32Type(4);
    const BITFIELD_PASSIVE_REF: U32Type = U32Type(5);
    const BITFIELD_ACTIVE_REF: U32Type = U32Type(6);
    const BITFIELD_DECLARATIVE_REF: U32Type = U32Type(7);

    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, bitfield) = U32Type::parse(bytes)?;

        match bitfield {
            Self::BITFIELD_ACTIVE0 => Active0FunctionsElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::Active0Functions(v))),
            Self::BITFIELD_ELEM_KIND_PASSIVE => {
                ElemKindPassiveFunctionsElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::ElemKindPassiveFunctions(v)))
            }
            Self::BITFIELD_ELEM_KIND_ACTIVE => {
                ElemKindActiveFunctionsElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::ElemKindActiveFunctions(v)))
            }
            Self::BITFIELD_ELEM_KIND_DECLARATIVE => {
                ElemKindDeclarativeFunctionsElementSegmentType::parse(bytes)
                    .map(|(b, v)| (b, ElementSegmentType::ElemKindDeclarativeFunctions(v)))
            }
            Self::BITFIELD_ACTIVE0_EXPR => Active0ExprElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::Active0Expr(v))),
            Self::BITFIELD_PASSIVE_REF => PassiveRefElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::PassiveRef(v))),
            Self::BITFIELD_ACTIVE_REF => ActiveRefElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::ActiveRef(v))),
            Self::BITFIELD_DECLARATIVE_REF => DeclarativeRefElementSegmentType::parse(bytes)
                .map(|(b, v)| (b, ElementSegmentType::DeclarativeRef(v))),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

pub struct Active0FunctionsElementSegmentType {
    pub mode: ElemModeActive0,
    // RefType::FuncRef only
    pub init: Vec<FuncIdx>,
}

impl Active0FunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, expression) = ExpressionType::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            Active0FunctionsElementSegmentType {
                mode: ElemModeActive0 { offset: expression },
                init,
            },
        ))
    }
}

pub struct ElemKindPassiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModePassive,
}

impl ElemKindPassiveFunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, elem_kind) = ElemKind::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            ElemKindPassiveFunctionsElementSegmentType {
                elem_kind,
                init,
                mode: ElemModePassive {},
            },
        ))
    }
}

pub struct ElemKindActiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModeActive,
}

impl ElemKindActiveFunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, table_idx) = U32Type::parse(bytes).map(|(b, v)| (b, TableIdx(v)))?;
        let (bytes, expression) = ExpressionType::parse(bytes)?;
        let (bytes, elem_kind) = ElemKind::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            ElemKindActiveFunctionsElementSegmentType {
                elem_kind,
                init,
                mode: ElemModeActive {
                    table_idx,
                    offset: expression,
                },
            },
        ))
    }
}

pub struct ElemKindDeclarativeFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModeDeclarative,
}

impl ElemKindDeclarativeFunctionsElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, elem_kind) = ElemKind::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<FuncIdx> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let func_idx_parsed = U32Type::parse(remaining_bytes).map(|r| (r.0, FuncIdx(r.1)))?;

            remaining_bytes = func_idx_parsed.0;
            init.push(func_idx_parsed.1);
        }

        Ok((
            remaining_bytes,
            ElemKindDeclarativeFunctionsElementSegmentType {
                elem_kind,
                init,
                mode: ElemModeDeclarative {},
            },
        ))
    }
}

pub struct Active0ExprElementSegmentType {
    pub elems: Vector<ExpressionType>,
    pub mode: ElemModeActive0,
}

pub struct PassiveRefElementSegmentType {
    pub elems: Vector<ExpressionType>,
    pub mode: ElemModePassive,
}

pub struct ActiveRefElementSegmentType {
    pub elems: Vector<ExpressionType>,
    pub mode: ElemModeActive,
}

pub struct DeclarativeRefElementSegmentType {
    pub elems: Vector<ExpressionType>,
    pub mode: ElemModeDeclarative,
}

pub struct ElemModeActive {
    pub table_idx: TableIdx,
    pub offset: ExpressionType,
}

pub struct ElemModeActive0 {
    pub offset: ExpressionType,
}
pub struct ElemModePassive;
pub struct ElemModeDeclarative;

pub enum ElemKind {
    FuncRef,
}

impl ElemKind {
    const ENCODE_BYTE_FUNC_REF: Byte = 0x00;

    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, elem_kind) = take(1usize)(bytes)?;

        match elem_kind[0] {
            Self::ENCODE_BYTE_FUNC_REF => Ok((bytes, ElemKind::FuncRef)),
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug)]
pub struct CodeType {
    pub size: U32Type,
    pub code: FuncCodeType,
}

#[derive(Debug)]
pub struct FuncCodeType {
    pub locals: Vec<LocalsType>,
    pub expression: ExpressionType,
}

#[derive(Debug)]
pub struct LocalsType {
    pub n: U32Type,
    pub val_types: Vec<ValType>,
}

impl LocalsType {
    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], LocalsType> {
        let (bytes, n) = U32Type::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let mut val_types: Vec<ValType> = Vec::with_capacity(n.0 as usize);

        for _ in 0..n.0 {
            let parsed_val_type = ValType::parse(remaining_bytes)?;
            remaining_bytes = parsed_val_type.0;
            val_types.push(parsed_val_type.1);
        }

        Ok((remaining_bytes, LocalsType { n, val_types }))
    }
}

pub enum DataType {
    Active0(Active0DataType),
    Active(ActiveDataType),
    Passive(PassiveDataType),
}

impl DataType {
    const BITFIELD_ACTIVE0: U32Type = U32Type(0);
    const BITFIELD_PASSIVE: U32Type = U32Type(1);
    const BITFIELD_ACTIVE: U32Type = U32Type(2);

    pub fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        unimplemented!()
    }
}

type Active0DataType = GenericDataType<DataModeActive0>;
type ActiveDataType = GenericDataType<DataModeActive>;
type PassiveDataType = GenericDataType<DataModePassive>;

pub struct DataModeActive0 {
    pub offset: ExpressionType,
}

pub struct DataModeActive {
    pub memory: MemIdx,
    pub offset: ExpressionType,
}

pub struct DataModePassive;

pub struct GenericDataType<Mode> {
    pub mode: Mode,
    pub init: Vector<Byte>,
}
