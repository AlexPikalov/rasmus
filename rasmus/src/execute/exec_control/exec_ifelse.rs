use syntax::module::IfElseInstructionType;

use crate::{
    instances::{stack::Stack, store::Store},
    result::RResult,
};

pub fn exec_ifelse(
    stack: &mut Stack,
    store: &mut Store,
    &IfElseInstructionType {
        ref blocktype,
        ref if_instructions,
        ref else_instructions,
    }: &IfElseInstructionType,
) -> RResult<()> {
    todo!()
}
