use syntax::types::LabelIdx;

use crate::{
    instances::{stack::Stack, store::Store},
    result::RResult,
};

pub fn exec_br(stack: &mut Stack, store: &mut Store, label_idx: &LabelIdx) -> RResult<()> {
    todo!()
}
