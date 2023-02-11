use std::rc::Rc;

use super::data::DataInst;
use super::elem::ElemInst;
use super::export::ExternVal;
use super::func::{Func, FuncInst, FuncInstLocal, HostCode, HostFunc};
use super::global::GlobalInst;
use super::memory::MemInst;
use super::module::ModuleInst;
use super::ref_inst::RefInst;
use super::table::TableInst;
use super::value::Val;
use crate::{
    address::*,
    result::{RResult, Trap},
};
use syntax::validation::types_validation::{is_memory_type_valid, is_table_type_valid};
use syntax::{
    instructions::ExpressionType,
    module::Module,
    types::{Byte, FuncType, GlobalType, MemType, RefType, TableType},
};

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

    pub fn allocate_local_func(&mut self, func: Func, module_inst: Rc<ModuleInst>) -> FuncAddr {
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

    pub fn allocate_table(&mut self, table_type: TableType, elem: RefInst) -> TableAddr {
        let len = table_type.limits.min.0 as usize;
        let table_inst = TableInst {
            table_type,
            elem: vec![elem; len],
        };
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

    // TODO: implement resolve_imports to get extern_vals (implement module registry)
    // TODO: implement resolve_globals to get globals values (according to the spec init of a global must be a single const instruction, take value from there)
    // TODO: implement resolve_elems to get refs vector of module's element segments
    pub fn allocate_module(
        &mut self,
        mut module: Module,
        extern_vals: Vec<ExternVal>,
        mut globals: Vec<Val>,
        mut refs: Vec<Vec<RefInst>>,
    ) -> RResult<Rc<ModuleInst>> {
        let mut module_inst = ModuleInst {
            types: vec![],
            tableaddrs: vec![],
            globaladdrs: vec![],
            funcaddrs: vec![],
            memaddrs: vec![],
            elemaddrs: vec![],
            dataaddrs: vec![],
            exports: vec![],
            start: None,
        };

        // table allocations
        for table_type in &module.tables {
            if !is_table_type_valid(&table_type) {
                return Err(Trap);
            }
        }
        for table_type in module.tables {
            let elem = RefInst::Null(table_type.element_ref_type.clone());
            module_inst
                .tableaddrs
                .push(self.allocate_table(table_type, elem));
        }
        module_inst
            .tableaddrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternVal::Table(addr) => Some(addr),
                _ => None,
            }));

        // mem allocations
        for mem_type in &module.mems {
            if !is_memory_type_valid(&mem_type) {
                return Err(Trap);
            }
        }
        for mem_type in module.mems {
            module_inst.memaddrs.push(self.allocate_mem(mem_type));
        }
        module_inst
            .memaddrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternVal::Mem(addr) => Some(addr),
                _ => None,
            }));

        // global allocations
        for global_type in module.globals {
            globals.rotate_left(1);
            let val = globals.pop().ok_or(Trap)?;
            module_inst
                .globaladdrs
                .push(self.allocate_global(global_type, val));
        }
        module_inst
            .globaladdrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternVal::Global(addr) => Some(addr),
                _ => None,
            }));

        // elem allocation
        for element_segment in module.elems {
            let elem_type = element_segment.get_type();
            refs.rotate_left(1);
            let elem = refs.pop().ok_or(Trap)?;
            self.allocate_elem(elem_type, elem);
        }

        // data allocation
        for mut data in module.datas {
            self.allocate_data(data.clone_data());
        }

        let mut module_inst_rc = Rc::new(module_inst);

        // func allocations
        for (i, type_idx) in module.funcs.iter_mut().enumerate() {
            let code = module.code.get_mut(i).ok_or(Trap)?;
            let locals =
                code.code
                    .locals
                    .clone()
                    .iter()
                    .fold(vec![], |mut locals_acc, current_locals| {
                        locals_acc.append(&mut vec![
                            current_locals.val_type.clone();
                            current_locals.n.0 as usize
                        ]);
                        locals_acc
                    });
            let body = ExpressionType {
                instructions: code.code.expression.instructions.clone(),
            };

            let func = Func {
                func_type: type_idx.clone(),
                locals,
                body,
            };

            let func_addr = self.allocate_local_func(func, module_inst_rc.clone());
            match Rc::get_mut(&mut module_inst_rc) {
                Some(inst) => inst.funcaddrs.push(func_addr),
                None => return Err(Trap),
            }
        }
        match Rc::get_mut(&mut module_inst_rc) {
            Some(inst) => inst
                .funcaddrs
                .extend(extern_vals.iter().filter_map(|v| match v {
                    ExternVal::Func(addr) => Some(addr),
                    _ => None,
                })),
            None => return Err(Trap),
        }

        Ok(module_inst_rc)
    }
}
