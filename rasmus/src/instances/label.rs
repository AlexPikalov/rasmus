use std::rc::Rc;

use syntax::module::InstructionType;

#[derive(Debug, Clone)]
pub struct LabelInst {
    pub arity: usize,
    pub instructions: Rc<Vec<InstructionType>>,
}
