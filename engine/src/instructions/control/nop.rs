use crate::instructions::{Instruction, InstructionResult, InstructionType};
use crate::types::Value;

pub struct NOP;

impl NOP {
    pub fn new() -> Self {
        NOP
    }
}

impl Instruction for NOP {
    fn get_type(&self) -> InstructionType {
        InstructionType {
            args: vec![],
            res: None,
        }
    }

    fn exec(&self, _args: Vec<Value>) -> Option<InstructionResult> {
        None
    }
}
