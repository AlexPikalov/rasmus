use std::rc::Rc;

use super::data::DataInst;
use super::elem::ElemInst;
use super::export::{ExportInst, ExternVal};
use super::func::{FuncInst, FuncInstLocal, HostCode, HostFunc};
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
    module::{ExportDescription, Module},
    types::{Byte, Func, FuncType, GlobalType, MemType, RefType, TableType},
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

    pub fn allocate_module_from(
        &mut self,
        mut base_module_instance: Rc<ModuleInst>,
        module: Module,
        extern_vals: Vec<ExternVal>,
        mut globals: Vec<Val>,
        mut refs: Vec<Vec<RefInst>>,
    ) -> RResult<Rc<ModuleInst>> {
        // table allocations
        for table_type in &module.tables {
            if !is_table_type_valid(&table_type) {
                return Err(Trap);
            }
        }
        let tableaddrs = module.tables.iter().map(|table_type| {
            let elem = RefInst::Null(table_type.element_ref_type.clone());
            self.allocate_table(table_type.clone(), elem)
        });
        match Rc::get_mut(&mut base_module_instance) {
            Some(inst) => {
                inst.tableaddrs.extend(tableaddrs);
                inst.tableaddrs
                    .extend(extern_vals.iter().filter_map(|v| match v {
                        ExternVal::Table(addr) => Some(addr),
                        _ => None,
                    }))
            }
            None => return Err(Trap),
        }

        // mem allocations
        for mem_type in &module.mems {
            if !is_memory_type_valid(&mem_type) {
                return Err(Trap);
            }
        }
        let tableaddrs = module
            .mems
            .iter()
            .map(|mem_type| self.allocate_mem(mem_type.clone()));
        match Rc::get_mut(&mut base_module_instance) {
            Some(inst) => inst.memaddrs.extend(tableaddrs),
            None => return Err(Trap),
        }

        // global allocations
        let mut globaladdrs = Vec::with_capacity(module.globals.len() + extern_vals.len());
        for global in &module.globals {
            globals.rotate_left(1);
            let val = globals.pop().ok_or(Trap)?;
            globaladdrs.push(self.allocate_global(global.global_type.clone(), val));
        }
        match Rc::get_mut(&mut base_module_instance) {
            Some(inst) => inst.globaladdrs.extend_from_slice(&globaladdrs),
            None => return Err(Trap),
        }

        // elem allocation
        for element_segment in &module.elems {
            let elem_type = element_segment.get_type();
            refs.rotate_left(1);
            let elem = refs.pop().ok_or(Trap)?;
            self.allocate_elem(elem_type, elem);
        }

        // data allocation
        for mut data in &module.datas {
            self.allocate_data(data.clone_data());
        }

        // exports instantiation
        for export_declaration in module.exports {
            let export_inst = ExportInst {
                name: export_declaration.name.clone(),
                value: match export_declaration.desc {
                    ExportDescription::Func(type_idx) => {
                        let funcaddr = base_module_instance
                            .funcaddrs
                            .get(type_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Func(funcaddr)
                    }
                    ExportDescription::Global(global_idx) => {
                        let globaladdr = base_module_instance
                            .globaladdrs
                            .get(global_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Global(globaladdr)
                    }
                    ExportDescription::Mem(mem_idx) => {
                        let memaddr = base_module_instance
                            .memaddrs
                            .get(mem_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Mem(memaddr)
                    }
                    ExportDescription::Table(table_idx) => {
                        let tableaddr = base_module_instance
                            .tableaddrs
                            .get(table_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Table(tableaddr)
                    }
                },
            };
            match Rc::get_mut(&mut base_module_instance) {
                Some(inst) => {
                    inst.exports.push(export_inst);
                }
                _ => return Err(Trap),
            }
        }

        Ok(base_module_instance)
    }

    // TODO: implement resolve_imports to get extern_vals (implement module registry)
    // TODO: implement resolve_globals to get globals values (according to the spec init of a global must be a single const instruction, take value from there)
    // TODO: implement resolve_elems to get refs vector of module's element segments
    pub fn allocate_module(
        &mut self,
        module: Module,
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
        for table_type in &module.tables {
            let elem = RefInst::Null(table_type.element_ref_type.clone());
            module_inst
                .tableaddrs
                .push(self.allocate_table(table_type.clone(), elem));
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
        for mem_type in &module.mems {
            module_inst
                .memaddrs
                .push(self.allocate_mem(mem_type.clone()));
        }
        module_inst
            .memaddrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternVal::Mem(addr) => Some(addr),
                _ => None,
            }));

        // global allocations
        for global in &module.globals {
            globals.rotate_left(1);
            let val = globals.pop().ok_or(Trap)?;
            module_inst
                .globaladdrs
                .push(self.allocate_global(global.global_type.clone(), val));
        }
        module_inst
            .globaladdrs
            .extend(extern_vals.iter().filter_map(|v| match v {
                ExternVal::Global(addr) => Some(addr),
                _ => None,
            }));

        // elem allocation
        for element_segment in &module.elems {
            let elem_type = element_segment.get_type();
            refs.rotate_left(1);
            let elem = refs.pop().ok_or(Trap)?;
            self.allocate_elem(elem_type, elem);
        }

        // data allocation
        for data in &module.datas {
            self.allocate_data(data.clone_data());
        }

        let mut module_inst_rc = Rc::new(module_inst);

        // func allocations
        let funcs = module.get_funcs().ok_or(Trap)?;
        for func in funcs {
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

        // exports instantiation
        for export_declaration in module.exports {
            let export_inst = ExportInst {
                name: export_declaration.name.clone(),
                value: match export_declaration.desc {
                    ExportDescription::Func(type_idx) => {
                        let funcaddr = module_inst_rc
                            .funcaddrs
                            .get(type_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Func(funcaddr)
                    }
                    ExportDescription::Global(global_idx) => {
                        let globaladdr = module_inst_rc
                            .globaladdrs
                            .get(global_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Global(globaladdr)
                    }
                    ExportDescription::Mem(mem_idx) => {
                        let memaddr = module_inst_rc
                            .memaddrs
                            .get(mem_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Mem(memaddr)
                    }
                    ExportDescription::Table(table_idx) => {
                        let tableaddr = module_inst_rc
                            .tableaddrs
                            .get(table_idx.0 .0 as usize)
                            .ok_or(Trap)?
                            .clone();
                        ExternVal::Table(tableaddr)
                    }
                },
            };
            match Rc::get_mut(&mut module_inst_rc) {
                Some(inst) => {
                    inst.exports.push(export_inst);
                }
                _ => return Err(Trap),
            }
        }

        Ok(module_inst_rc)
    }
}
