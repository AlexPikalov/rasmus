use super::value::Val;
use syntax::types::GlobalType;

pub struct GlobalInst {
    pub global_type: GlobalType,
    pub value: Val,
}
