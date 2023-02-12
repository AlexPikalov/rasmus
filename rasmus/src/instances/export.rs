use crate::address::*;
use syntax::types::NameType;

#[derive(Debug)]
pub struct ExportInst {
    pub name: NameType,
    pub value: ExternVal,
}

#[derive(Debug)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}
