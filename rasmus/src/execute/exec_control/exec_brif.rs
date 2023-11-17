use syntax::types::LabelIdx;

use crate::{
    instances::{stack::Stack, store::Store},
    result::RResult,
};

pub fn exec_brif(stack: &mut Stack, store: &mut Store, label_idx: &LabelIdx) -> RResult<()> {
    todo!()
}
