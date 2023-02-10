use crate::address::{ExternAddr, FuncAddr};
use syntax::types::RefType;

#[derive(Debug, Clone)]
pub enum RefInst {
    Null(RefType),
    Func(FuncAddr),
    Extern(ExternAddr),
}
