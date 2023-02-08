use super::frame::Frame;
use super::label::LabelInst;
use crate::address::{ExternAddr, FuncAddr};

#[derive(Debug)]
pub enum InstructionInst {
    Trap,
    Ref(FuncAddr),
    RefExtern(ExternAddr),
    Invoke(FuncAddr),
    Label(LabelInst),
    Frame(Frame),
    End,
}
