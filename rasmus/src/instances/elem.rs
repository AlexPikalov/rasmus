use super::ref_inst::RefInst;
use syntax::types::RefType;

pub struct ElemInst {
    pub elem_type: RefType,
    pub elem: Vec<RefInst>,
}
