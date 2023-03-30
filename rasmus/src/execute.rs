use crate::result::{RResult, Trap};

use crate::instances::frame::Frame;
use crate::instances::stack::{Stack, StackEntry};
use crate::instances::store::Store;
use crate::instances::value::Val;
use syntax::instructions::{ExpressionType, InstructionType};

pub fn execute_expression(
    expr: &ExpressionType,
    stack: &mut Stack,
    store: &mut Store,
) -> RResult<Val> {
    let frame = match stack.last().cloned() {
        Some(StackEntry::Frame(frame)) => frame,
        _ => return Err(Trap),
    };

    for ref instr in &expr.instructions {
        execute_instruction(instr, stack, store)?;
    }

    stack.pop_value().ok_or(Trap)
}

pub fn execute_instruction(
    instr: &InstructionType,
    stack: &mut Stack,
    store: &mut Store,
    // frame_ref: &Frame,
) -> RResult<()> {
    unimplemented!()
}
