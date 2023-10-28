use crate::address::*;
use syntax::types::NameType;

#[derive(Debug, PartialEq)]
pub struct ExportInst {
    pub name: NameType,
    pub value: ExternVal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}
