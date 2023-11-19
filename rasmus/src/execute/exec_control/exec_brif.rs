use syntax::{module::InstructionType, types::LabelIdx};

use crate::{
    instances::{stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::exec_br;

pub fn exec_brif(
    stack: &mut Stack,
    store: &mut Store,
    label_idx: &LabelIdx,
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<()> + Copy,
) -> RResult<()> {
    if stack.pop_i32().ok_or(Trap)? != 0 {
        return exec_br(stack, store, label_idx, execute_instruction_fn);
    }

    Ok(())
}
