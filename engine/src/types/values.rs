use super::addrs::{ExternAddr, FuncAddr};

pub enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

pub enum VecType {
    V128,
}

pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

pub enum RefType {
    FuncRef,
    ExternRef,
}

pub enum Value {
    Numeric(Numeric),
    Vector(Vector),
    Reference(Reference),
}

pub enum Numeric {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Numeric {
    const DEFAULT_I32: Numeric = Numeric::I32(0);
    const DEFAULT_I64: Numeric = Numeric::I64(0);
    const DEFAULT_F32: Numeric = Numeric::F32(0.0);
    const DEFAULT_F64: Numeric = Numeric::F64(0.0);
}

pub struct Vector(i128);

impl Vector {
    const DEFAULT_VECTOR: Vector = Vector(0);
}

pub enum Reference {
    Null,
    FuncAddr(FuncAddr),
    ExternAddr(ExternAddr),
}

impl Reference {
    const DEFAULT_REFERENCE: Reference = Reference::Null;
}
