use crate::address::{ExternAddr, FuncAddr};
use syntax::types::RefType;

pub enum RefInst {
    Null(RefType),
    Func(FuncAddr),
    Extern(ExternAddr),
}
