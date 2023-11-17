use syntax::types::LabelIdx;

use crate::{
    instances::{stack::Stack, store::Store},
    result::RResult,
};

pub fn exec_brtable(
    stack: &mut Stack,
    store: &mut Store,
    brtable_arg: &(Vec<LabelIdx>, LabelIdx),
) -> RResult<()> {
    todo!()
}
