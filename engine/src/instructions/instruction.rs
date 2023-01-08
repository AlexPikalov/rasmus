use crate::types::{Trap, ValType, Value};

pub enum InstructionResult {
    Value(Value),
    Trap(Trap),
}

pub enum ResultType {
    Value(ValType),
    Trap,
}

pub struct InstructionType {
    pub args: Vec<ValType>,
    pub res: Option<ResultType>,
}

pub trait Instruction {
    fn get_type(&self) -> InstructionType;
    fn exec(&self, args: Vec<Value>) -> Option<InstructionResult>;
}
