use crate::address::*;
use crate::result::RResult;
use syntax::module::*;
use syntax::types::*;

use super::export::ExportInst;

#[derive(Debug)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub funcaddrs: Vec<FuncAddr>,
    pub tableaddrs: Vec<TableAddr>,
    pub memaddrs: Vec<MemAddr>,
    pub globaladdrs: Vec<GlobalAddr>,
    pub elemaddrs: Vec<ElemAddr>,
    pub dataaddrs: Vec<DataAddr>,
    pub exports: Vec<ExportInst>,
    start: Option<StartType>,
}
