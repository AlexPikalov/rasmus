use super::frame::Frame;
use super::label::LabelInst;
use super::value::Val;

#[derive(Debug)]
pub enum StackEntry {
    Value(Val),
    Label(LabelInst),
    Frame(Frame),
}

pub struct Stack {
    stack: Vec<StackEntry>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: vec![] }
    }

    pub fn push_entry(&mut self, entry: StackEntry) {
        self.stack.push(entry);
    }
}
