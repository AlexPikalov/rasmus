pub use super::instructions::*;
use super::types::*;

pub struct Module {
    pub custom_section: Section<CustomSection>,
    pub type_section: Section<Vector<FuncType>>,
    pub import_section: Section<Vector<ImportType>>,
    pub func_section: Section<Vector<TypeIdx>>,
    pub table_section: Section<Vector<TableType>>,
    pub memory_section: Section<Vector<MemType>>,
    pub global_section: Section<Vector<GlobalType>>,
    pub export_section: Section<Vector<ExportType>>,
    pub start_section: Section<Vector<StartType>>,
    pub element_section: Section<Vector<ElementSegmentType>>,
    pub code_section: Section<Vector<CodeType>>,
    pub data_section: Section<Vector<DataType>>,
    pub data_count_section: Section<U32Type>,
}

pub type SectionIdValue = Byte;

pub struct Section<T> {
    pub section_id: SectionIdValue,
    pub size: U32Type,
    pub cont: T,
}

enum SectionId {
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

pub struct CustomSection {
    pub name: String,
    pub bytes: Vector<Byte>,
}

pub struct ImportType {
    pub module: NameType,
    pub name: NameType,
    pub desc: ImportDescription,
}

pub enum ImportDescription {
    Func(TypeIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

impl ImportDescription {
    const ENCODE_BYTE_FUNC: Byte = 0x00;
    const ENCODE_BYTE_TABLE: Byte = 0x01;
    const ENCODE_BYTE_MEM: Byte = 0x02;
    const ENCODE_BYTE_GLOBAL: Byte = 0x03;
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
    const ENCODE_BYTE_FUNC: Byte = 0x00;
    const ENCODE_BYTE_TABLE: Byte = 0x01;
    const ENCODE_BYTE_MEM: Byte = 0x02;
    const ENCODE_BYTE_GLOBAL: Byte = 0x03;
}

pub struct StartType {
    pub func: FuncIdx,
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
}

pub struct Active0FunctionsElementSegmentType {
    pub mode: ElemModeActive0,
    // RefType::FuncRef only
    pub init: Vector<RefType>,
}

pub struct ElemKindPassiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vector<RefType>,
    pub mode: ElemModePassive,
}

pub struct ElemKindActiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vector<RefType>,
    pub mode: ElemModeActive,
}

pub struct ElemKindDeclarativeFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vector<RefType>,
    pub mode: ElemModeDeclarative,
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
}

pub struct CodeType {
    pub size: U32Type,
    pub code: FuncCodeType,
}

pub struct FuncCodeType {
    pub locals: Vector<LocalsType>,
    pub expression: ExpressionType,
}

pub struct LocalsType {
    pub n: U32Type,
    pub val_types: Vec<ValType>,
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
