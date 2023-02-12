use std::rc::Rc;

use crate::address::*;
use crate::instances::{frame::Frame, stack::Stack, stack::StackEntry, store::Store};
use crate::module_registry::ModuleRegistry;
use crate::result::{RResult, Trap};
use syntax::module::*;
use syntax::types::*;

use super::export::{ExportInst, ExternVal};

#[derive(Debug, Default)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub funcaddrs: Vec<FuncAddr>,
    pub tableaddrs: Vec<TableAddr>,
    pub memaddrs: Vec<MemAddr>,
    pub globaladdrs: Vec<GlobalAddr>,
    pub elemaddrs: Vec<ElemAddr>,
    pub dataaddrs: Vec<DataAddr>,
    pub exports: Vec<ExportInst>,
    pub start: Option<StartType>,
}
/// 1 create base instance
/// 2. allocate funcs into base instance
/// 3. copy func addrs from base instance to aux module
impl ModuleInst {
    pub fn instantiate(
        store: &mut Store,
        stack: &mut Stack,
        module: Module,
        module_registry: &Box<ModuleRegistry>,
    ) -> RResult<Self> {
        if !module.is_valid() {
            return Err(Trap);
        }

        let externals = module_registry.resolve_imports(&module);

        let mut aux_module = Rc::new(ModuleInst {
            globaladdrs: externals
                .into_iter()
                .filter_map(|external| match external.value {
                    ExternVal::Global(a) => Some(a),
                    _ => None,
                })
                .collect(),
            ..Default::default()
        });

        for func in module.get_funcs().ok_or(Trap)? {
            let module_ref = aux_module.clone();
            match Rc::get_mut(&mut aux_module) {
                Some(inst) => {
                    inst.funcaddrs
                        .push(store.allocate_local_func(func, module_ref));
                }
                None => return Err(Trap),
            }
        }

        stack.push_entry(StackEntry::Frame(Frame {
            module: aux_module.clone(),
            locals: vec![],
        }));
    }
}
