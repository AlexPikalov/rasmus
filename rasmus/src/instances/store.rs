use std::rc::Rc;

use super::data::DataInst;
use super::elem::ElemInst;
use super::func::{Func, FuncInst, FuncInstLocal, HostCode, HostFunc};
use super::global::GlobalInst;
use super::memory::MemInst;
use super::module::ModuleInst;
use super::table::TableInst;
use crate::address::*;
use syntax::types::FuncType;

pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
    pub elems: Vec<ElemInst>,
    pub datas: Vec<DataInst>,
}

impl Store {
    /// Create empy store
    pub fn new() -> Store {
        Store {
            funcs: vec![],
            tables: vec![],
            mems: vec![],
            globals: vec![],
            elems: vec![],
            datas: vec![],
        }
    }

    pub fn allocate_local_func(&mut self, func: Func, module_inst: &Rc<ModuleInst>) -> FuncAddr {
        let func_type = module_inst.types[func.func_type.0 .0 as usize].clone();
        let func_inst = FuncInst::FuncInst(FuncInstLocal {
            func_type,
            module: module_inst.clone(),
            code: func,
        });
        self.funcs.push(func_inst);

        self.funcs.len() - 1 as FuncAddr
    }

    pub fn allocate_host_func(&mut self, func_type: FuncType, host_code: HostCode) -> FuncAddr {
        let func_inst = FuncInst::HostFunc(HostFunc {
            func_type,
            host_code,
        });

        self.funcs.push(func_inst);

        self.funcs.len() - 1 as FuncAddr
    }
}
