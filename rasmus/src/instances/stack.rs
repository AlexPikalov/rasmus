use syntax::types::U32Type;

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

    pub fn pop_i32(&mut self) -> Option<u32> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::I32(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_i64(&mut self) -> Option<u64> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::I64(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_f32(&mut self) -> Option<f32> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::F32(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_f64(&mut self) -> Option<f64> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::F64(v))) => Some(v),
            Some(v) => {
                self.stack.push(v);
                None
            }
            None => None,
        }
    }

    pub fn pop_v128(&mut self) -> Option<u128> {
        match self.stack.pop() {
            Some(StackEntry::Value(Val::Vec(v))) => Some(v),
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

    pub fn current_frame(&mut self) -> Option<&mut Frame> {
        self.stack.iter_mut().rev().find_map(|entry| match entry {
            StackEntry::Frame(frame) => Some(frame),
            _ => None,
        })
    }
}
