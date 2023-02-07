use crate::address::*;
use syntax::types::NameType;

pub struct ExportInst {
    pub name: NameType,
}

pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}
