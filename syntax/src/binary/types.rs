pub type Byte = u8;

pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

pub enum VecType {
    V128,
}

impl VecType {
    const ENCODE_BYTE: Byte = 0x7B;
}

pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl NumType {
    const ENCODE_BYTE_I32: Byte = 0x7F;
    const ENCODE_BYTE_I64: Byte = 0x7E;
    const ENCODE_BYTE_F32: Byte = 0x7D;
    const ENCODE_BYTE_F64: Byte = 0x7C;
}

pub enum RefType {
    FuncRef,
    ExternRef,
}

impl RefType {
    const ENCODE_BYTE_FUNC_REF: Byte = 0x70;
    const ENCODE_BYTE_EXTERN_REF: Byte = 0x6F;
}

pub struct FuncType {
    pub parameters: Vector<ValType>,
    pub results: Vector<ValType>,
}

impl FuncType {
    const ENCODE_BYTE: Byte = 0x60;
}

pub struct Vector<T> {
    pub length: U32Type,
    pub elements: Vec<T>,
}

// NOTE: UTF-8 bytes only
pub struct NameType(pub Vector<Byte>);

pub struct TableType {
    pub limit: LimitsType,
    pub element_ref_type: RefType,
}

pub struct LimitsType {
    pub min: U32Type,
    pub max: Option<U32Type>,
}

impl LimitsType {
    const ENCODE_BYTE_MAX_NOT_PRESENT: Byte = 0x00;
    const ENCODE_BYTE_MAX_PRESENT: Byte = 0x01;
}

pub struct MemType {
    pub limits: LimitsType,
}

pub struct GlobalType {
    pub mut_type: MutType,
    pub val_type: ValType,
}

pub enum MutType {
    Const,
    Var,
}

impl MutType {
    const ENCODE_BYTE_CONST: Byte = 0x00;
    const ENCODE_BYTE_VAR: Byte = 0x01;
}

pub struct TypeIdx(U32Type);
pub struct FuncIdx(U32Type);
pub struct TableIdx(U32Type);
pub struct MemIdx(U32Type);
pub struct GlobalIdx(U32Type);
pub struct ElemIdx(U32Type);
pub struct DataIdx(U32Type);
pub struct LocalIdx(U32Type);
pub struct LabelIdx(U32Type);
pub struct LaneIdx(Byte);

pub struct U32Type(pub u32);
pub struct S33Type(pub i64);
pub struct I32Type(pub i32);
pub struct I64Type(pub i64);
pub struct F32Type(pub f32);

pub struct F64Type(pub f64);

pub type ParseResult<T> = Result<T, SyntaxError>;

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedSectionIdValue,
}
