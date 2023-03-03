use std::rc::Rc;

use super::instruction::InstructionInst;

#[derive(Debug, Clone)]
pub struct LabelInst {
    pub arity: usize,
    pub instructions: Rc<Vec<InstructionInst>>,
}
