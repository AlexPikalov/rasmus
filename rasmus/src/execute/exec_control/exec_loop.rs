use syntax::module::LoopInstructionType;

use crate::{
    instances::{stack::Stack, store::Store},
    result::RResult,
};

pub fn exec_loop(
    stack: &mut Stack,
    store: &mut Store,
    &LoopInstructionType {
        ref blocktype,
        ref instructions,
    }: &LoopInstructionType,
) -> RResult<()> {
    todo!()
}
