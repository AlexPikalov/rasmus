pub use super::instructions::*;
use super::types::*;
use crate::binary::{
    parse_trait::ParseWithNom,
    syntax_error::{ParseResult, SyntaxError},
};

use nom::{bytes::complete::take, IResult as NomResult};

#[derive(Debug, Default)]
pub struct Module {
    pub types: Vec<FuncType>,
    pub imports: Vec<ImportType>,
    // TODO: rewrite to func
    pub funcs: Vec<TypeIdx>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<Global>,
    pub exports: Vec<ExportType>,
    pub start: Option<StartType>,
    pub elems: Vec<ElementSegmentType>,
    pub code: Vec<CodeType>,
    pub datas: Vec<DataType>,
}

impl Module {
    pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

    pub fn is_valid(&self) -> bool {
        // TODO: validate according to
        // so it is guarated that element expression list always reduces to a reference value
        true
    }

    pub fn get_funcs(&self) -> Option<Vec<Func>> {
        let num = self.funcs.len();
        let mut funcs = Vec::with_capacity(num);
        for i in 0..num {
            let type_idx = self.funcs.get(i)?;
            let code = self.code.get(i)?;
            let locals =
                code.code
                    .locals
                    .clone()
                    .iter()
                    .fold(vec![], |mut locals_acc, current_locals| {
                        locals_acc.append(&mut vec![
                            current_locals.val_type.clone();
                            current_locals.n.0 as usize
                        ]);
                        locals_acc
                    });
            let body = ExpressionType {
                instructions: code.code.expression.instructions.clone(),
            };

            funcs.push(Func {
                func_type: type_idx.clone(),
                locals,
                body,
            });
        }

        Some(funcs)
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

#[derive(Debug, PartialEq)]
pub struct ImportType {
    pub module: NameType,
    pub name: NameType,
    pub desc: ImportDescription,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct ExportType {
    pub name: NameType,
    pub desc: ExportDescription,
}

#[derive(Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct StartType {
    pub func: FuncIdx,
}

impl ParseWithNom for StartType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        U32Type::parse(bytes).map(|(b, val)| (b, StartType { func: FuncIdx(val) }))
    }
}

#[derive(Debug)]
pub struct Global {
    pub global_type: GlobalType,
    pub init: ExpressionType,
}

impl ParseWithNom for Global {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, global_type) = GlobalType::parse(bytes)?;
        let (bytes, init) = ExpressionType::parse(bytes)?;

        Ok((bytes, Global { global_type, init }))
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn get_type(&self) -> RefType {
        match self {
            Self::Active0Functions(_) => RefType::FuncRef,
            Self::ElemKindPassiveFunctions(t) => (&t.elem_kind).into(),
            Self::ElemKindActiveFunctions(t) => (&t.elem_kind).into(),
            Self::ElemKindDeclarativeFunctions(t) => (&t.elem_kind).into(),
            Self::Active0Expr(_) => RefType::FuncRef,
            Self::PassiveRef(t) => t.ref_type.clone(),
            Self::ActiveRef(t) => t.ref_type.clone(),
            Self::DeclarativeRef(t) => t.ref_type.clone(),
        }
    }

    pub fn get_init(&self) -> &Vec<ExpressionType> {
        // TODO:
        unimplemented!()
    }

    pub fn get_offset(&self) -> &ExpressionType {
        unimplemented!()
    }

    pub fn get_table_idx(&self) -> TableIdx {
        unimplemented!()
    }
}

impl ParseWithNom for ElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
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

#[derive(Debug, PartialEq)]
pub struct Active0FunctionsElementSegmentType {
    pub mode: ElemModeActive0,
    // RefType::FuncRef only
    pub init: Vec<FuncIdx>,
}

impl ParseWithNom for Active0FunctionsElementSegmentType {
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

#[derive(Debug, PartialEq)]
pub struct ElemKindPassiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModePassive,
}

impl ParseWithNom for ElemKindPassiveFunctionsElementSegmentType {
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

#[derive(Debug, PartialEq)]
pub struct ElemKindActiveFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModeActive,
}

impl ParseWithNom for ElemKindActiveFunctionsElementSegmentType {
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

#[derive(Debug, PartialEq)]
pub struct ElemKindDeclarativeFunctionsElementSegmentType {
    pub elem_kind: ElemKind,
    pub init: Vec<FuncIdx>,
    pub mode: ElemModeDeclarative,
}

impl ParseWithNom for ElemKindDeclarativeFunctionsElementSegmentType {
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

#[derive(Debug, PartialEq)]
pub struct Active0ExprElementSegmentType {
    pub init: Vec<ExpressionType>,
    pub mode: ElemModeActive0,
}

impl ParseWithNom for Active0ExprElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, offset) = ExpressionType::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            Active0ExprElementSegmentType {
                init,
                mode: ElemModeActive0 { offset },
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct PassiveRefElementSegmentType {
    pub ref_type: RefType,
    pub init: Vec<ExpressionType>,
    pub mode: ElemModePassive,
}

impl ParseWithNom for PassiveRefElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, ref_type) = RefType::parse(bytes)?;

        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            PassiveRefElementSegmentType {
                ref_type,
                init,
                mode: ElemModePassive {},
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct ActiveRefElementSegmentType {
    pub ref_type: RefType,
    pub init: Vec<ExpressionType>,
    pub mode: ElemModeActive,
}

impl ParseWithNom for ActiveRefElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, table_idx) = U32Type::parse(bytes)?;
        let (bytes, offset) = ExpressionType::parse(bytes)?;
        let (bytes, ref_type) = RefType::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            ActiveRefElementSegmentType {
                ref_type,
                init,
                mode: ElemModeActive {
                    table_idx: TableIdx(table_idx),
                    offset,
                },
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct DeclarativeRefElementSegmentType {
    pub ref_type: RefType,
    pub init: Vec<ExpressionType>,
    pub mode: ElemModeDeclarative,
}

impl ParseWithNom for DeclarativeRefElementSegmentType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, ref_type) = RefType::parse(bytes)?;
        let mut remaining_bytes = bytes;
        let vector_len_parsed = U32Type::parse(remaining_bytes)?;
        remaining_bytes = vector_len_parsed.0;
        let vector_len = vector_len_parsed.1 .0 as usize;
        let mut init: Vec<ExpressionType> = Vec::with_capacity(vector_len);

        for _ in 0..vector_len {
            let init_expression_parsed = ExpressionType::parse(remaining_bytes)?;

            remaining_bytes = init_expression_parsed.0;
            init.push(init_expression_parsed.1);
        }

        Ok((
            remaining_bytes,
            DeclarativeRefElementSegmentType {
                ref_type,
                init,
                mode: ElemModeDeclarative {},
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct ElemModeActive {
    pub table_idx: TableIdx,
    pub offset: ExpressionType,
}

#[derive(Debug, PartialEq)]
pub struct ElemModeActive0 {
    pub offset: ExpressionType,
}

#[derive(Debug, PartialEq)]
pub struct ElemModePassive;

#[derive(Debug, PartialEq)]
pub struct ElemModeDeclarative;

#[derive(Debug, PartialEq)]
pub enum ElemKind {
    FuncRef,
}

impl ElemKind {
    const ENCODE_BYTE_FUNC_REF: Byte = 0x00;
}

impl Into<RefType> for &ElemKind {
    fn into(self) -> RefType {
        RefType::FuncRef
    }
}

impl ParseWithNom for ElemKind {
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

#[derive(Debug, PartialEq)]
pub struct CodeType {
    pub size: U32Type,
    pub code: FuncCodeType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncCodeType {
    pub locals: Vec<LocalsType>,
    pub expression: ExpressionType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalsType {
    pub n: U32Type,
    pub val_type: ValType,
}

impl ParseWithNom for LocalsType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], LocalsType> {
        let (bytes, n) = U32Type::parse(bytes)?;
        let (bytes, val_type) = ValType::parse(bytes)?;

        Ok((bytes, LocalsType { n, val_type }))
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    Active0(Active0DataType),
    Active(ActiveDataType),
    Passive(PassiveDataType),
}

impl DataType {
    const BITFIELD_ACTIVE0: U32Type = U32Type(0);
    const BITFIELD_PASSIVE: U32Type = U32Type(1);
    const BITFIELD_ACTIVE: U32Type = U32Type(2);

    pub fn clone_data(&self) -> Vec<Byte> {
        match self {
            Self::Active0(t) => t.init.clone(),
            Self::Active(t) => t.init.clone(),
            Self::Passive(t) => t.init.clone(),
        }
    }
}

impl ParseWithNom for DataType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, bitfield) = U32Type::parse(bytes)?;

        match bitfield {
            Self::BITFIELD_ACTIVE0 => {
                Ok(Active0DataType::parse(bytes).map(|(b, v)| (b, DataType::Active0(v)))?)
            }
            Self::BITFIELD_ACTIVE => {
                Ok(ActiveDataType::parse(bytes).map(|(b, v)| (b, DataType::Active(v)))?)
            }
            Self::BITFIELD_PASSIVE => {
                Ok(PassiveDataType::parse(bytes).map(|(b, v)| (b, DataType::Passive(v)))?)
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

type Active0DataType = GenericDataType<DataModeActive0>;
type ActiveDataType = GenericDataType<DataModeActive>;
type PassiveDataType = GenericDataType<DataModePassive>;

#[derive(Debug, Clone)]
pub struct DataModeActive0 {
    pub offset: ExpressionType,
}

impl ParseWithNom for DataModeActive0 {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        ExpressionType::parse(bytes).map(|(b, offset)| (b, DataModeActive0 { offset }))
    }
}

#[derive(Debug, Clone)]
pub struct DataModeActive {
    pub memory: MemIdx,
    pub offset: ExpressionType,
}

impl ParseWithNom for DataModeActive {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, memory) = U32Type::parse(bytes).map(|(b, v)| (b, MemIdx(v)))?;
        ExpressionType::parse(bytes).map(|(b, offset)| (b, DataModeActive { memory, offset }))
    }
}

#[derive(Debug, Clone)]
pub struct DataModePassive;

impl ParseWithNom for DataModePassive {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        Ok((bytes, DataModePassive {}))
    }
}

#[derive(Clone, Debug)]
pub struct GenericDataType<Mode: ::std::fmt::Debug + Clone> {
    pub mode: Mode,
    pub init: Vec<Byte>,
}

impl<Mode: ParseWithNom + std::fmt::Debug + Clone> GenericDataType<Mode> {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, mode) = Mode::parse(bytes)?;
        let (bytes, init_len) = U32Type::parse(bytes)?;
        take(init_len.0 as usize)(bytes).map(|(b, init)| {
            (
                b,
                GenericDataType {
                    mode,
                    init: init.to_vec(),
                },
            )
        })
    }
}
