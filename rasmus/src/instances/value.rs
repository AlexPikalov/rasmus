use super::ref_inst::RefInst;

pub enum Val {
    Num(NumInst),
    Vec(i128),
    Ref(RefInst),
}

pub enum NumInst {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}
