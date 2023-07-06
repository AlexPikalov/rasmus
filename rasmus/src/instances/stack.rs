use super::frame::Frame;
use super::label::LabelInst;
use super::value::Val;

#[derive(Debug, Clone)]
pub enum StackEntry {
    Value(Val),
    Label(LabelInst),
    Frame(Frame),
}

impl StackEntry {
    pub fn is_value(&self) -> bool {
        match self {
            StackEntry::Value(_) => true,
            _ => false,
        }
    }

    pub fn is_label(&self) -> bool {
        match self {
            StackEntry::Label(_) => true,
            _ => false,
        }
    }

    pub fn is_frame(&self) -> bool {
        match self {
            StackEntry::Frame(_) => true,
            _ => false,
        }
    }
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

    pub fn last(&self) -> Option<&StackEntry> {
        self.stack.last()
    }

    pub fn pop_value(&mut self) -> Option<Val> {
        match self.stack.pop() {
            Some(StackEntry::Value(val)) => Some(val),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        if let Some(stack_entry) = self.stack.last() {
            if stack_entry.is_frame() {
                if let Some(StackEntry::Frame(frame)) = self.stack.pop() {
                    return Some(frame);
                }
            }
        }

        None
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        self.stack.iter().rev().find_map(|entry| match entry {
            StackEntry::Frame(frame) => Some(frame),
            _ => None,
        })
    }
}
