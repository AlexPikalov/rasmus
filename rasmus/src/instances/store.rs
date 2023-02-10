use std::rc::Rc;

use super::data::DataInst;
use super::elem::ElemInst;
use super::func::{Func, FuncInst, FuncInstLocal, HostCode, HostFunc};
use super::global::GlobalInst;
use super::memory::MemInst;
use super::module::ModuleInst;
use super::ref_inst::RefInst;
use super::table::TableInst;
use super::value::Val;
use crate::address::*;
use syntax::types::{Byte, FuncType, GlobalType, MemType, RefType, TableType};

// #[derive(Debug)]
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

    pub fn allocate_table(&mut self, table_type: TableType, elem: Vec<RefInst>) -> TableAddr {
        let table_inst = TableInst { table_type, elem };
        self.tables.push(table_inst);

        self.tables.len() - 1 as TableAddr
    }

    pub fn allocate_mem(&mut self, mem_type: MemType) -> MemAddr {
        let size = (mem_type.limits.min.0.clone() as usize) * 2usize.pow(16);
        let mem_inst = MemInst {
            mem_type,
            data: vec![0x00; size],
        };

        self.mems.push(mem_inst);

        self.mems.len() - 1 as MemAddr
    }

    pub fn allocate_global(&mut self, global_type: GlobalType, value: Val) -> GlobalAddr {
        let global_inst = GlobalInst { global_type, value };
        self.globals.push(global_inst);

        self.globals.len() - 1 as GlobalAddr
    }

    pub fn allocate_elem(&mut self, elem_type: RefType, elem: Vec<RefInst>) -> ElemAddr {
        let elem_inst = ElemInst { elem, elem_type };
        self.elems.push(elem_inst);

        self.elems.len() - 1 as ElemAddr
    }

    pub fn allocate_data(&mut self, data: Vec<Byte>) -> DataAddr {
        let data_inst = DataInst { data };
        self.datas.push(data_inst);

        self.datas.len() - 1 as DataAddr
    }
}
