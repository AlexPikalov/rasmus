use crate::result::{RResult, Trap};

use crate::instances::stack::Stack;
use crate::instances::store::Store;
use syntax::instructions::{ExpressionType, InstructionType};

pub fn execute_expression(
    expr: ExpressionType,
    stack: &mut Stack,
    store: &mut Store,
) -> RResult<()> {
    for ref instr in expr.instructions {
        execute_instruction(instr, stack, store)?;
    }

    Ok(())
}

pub fn execute_instruction(
    instr: &InstructionType,
    stack: &mut Stack,
    store: &mut Store,
) -> RResult<()> {
    unimplemented!()
}
