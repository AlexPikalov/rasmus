use syntax::module::BlockInstructionType;

use crate::{
    instances::{stack::Stack, store::Store},
    result::RResult,
};

pub fn block(
    stack: &mut Stack,
    store: &mut Store,
    &BlockInstructionType {
        ref blocktype,
        ref instructions,
    }: &BlockInstructionType,
) -> RResult<()> {
    todo!()
}
