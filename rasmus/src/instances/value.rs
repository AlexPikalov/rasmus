use super::ref_inst::RefInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
    // Num(NumInst),
    Vec(i128),
    Ref(RefInst),
}
