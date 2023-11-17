use syntax::types::{TableIdx, TypeIdx};

use crate::{instances::stack::Stack, result::RResult};

pub fn exec_call_indirect(
    stack: &mut Stack,
    call_indirect_args: &(TypeIdx, TableIdx),
) -> RResult<()> {
    todo!()
}
