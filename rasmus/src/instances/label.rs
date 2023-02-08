use super::instruction::InstructionInst;

#[derive(Debug)]
pub struct LabelInst {
    pub arity: usize,
    pub instructions: Vec<InstructionInst>,
}
