use super::instructions::ExpressionType;

pub type Byte = u8;

#[derive(Debug, PartialEq, Clone)]
pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

impl ValType {
    pub fn is_num(&self) -> bool {
        if let ValType::NumType(_) = self {
            return true;
        }

        false
    }

    pub fn is_vec(&self) -> bool {
        if let ValType::VecType(_) = self {
            return true;
        }

        false
    }

    pub fn is_ref(&self) -> bool {
        if let ValType::RefType(_) = self {
            return true;
        }

        false
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

#[derive(Debug, PartialEq, Clone)]
pub struct ResultType(pub Vec<ValType>);

#[derive(Debug, PartialEq, Clone)]
pub enum VecType {
    V128,
}

impl VecType {}

#[derive(Debug, PartialEq, Clone)]
pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

impl NumType {
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
    pub fn get_all() -> Vec<Self> {
        vec![RefType::FuncRef, RefType::ExternRef]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncType {
    pub parameters: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
    pub const ENCODE_BYTE_FUNC: Byte = 0x60;
}

#[derive(Debug)]
pub struct Vector<T: std::fmt::Debug> {
    pub length: U32Type,
    pub elements: Vec<T>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NameType(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct TableType {
    pub limits: LimitsType,
    pub element_ref_type: RefType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LimitsType {
    pub min: U32Type,
    pub max: Option<U32Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemType {
    pub limits: LimitsType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalType {
    pub mut_type: MutType,
    pub val_type: ValType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MutType {
    Const,
    Var,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct FuncIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct TableIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct MemIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct ElemIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct DataIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct LocalIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct LabelIdx(pub U32Type);

#[derive(Debug, PartialEq, Clone)]
pub struct LaneIdx(pub Byte);

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct U32Type(pub u32);

#[derive(Debug, PartialEq, Clone)]
pub struct S33Type(pub i64);

#[derive(Debug, PartialEq, Clone)]
pub struct I32Type(pub u32);

#[derive(Debug, PartialEq, Clone)]
pub struct I64Type(pub u64);

#[derive(Debug, PartialEq, Clone)]
pub struct F32Type(pub f32);

#[derive(Debug, PartialEq, Clone)]
pub struct F64Type(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub func_type: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: ExpressionType,
}
