use std::rc::Rc;

use crate::address::*;
use crate::execute::execute_expression;
use crate::instances::{frame::Frame, stack::Stack, stack::StackEntry, store::Store};
use crate::module_registry::ModuleRegistry;
use crate::result::{RResult, Trap};
use syntax::module::*;
use syntax::types::*;

use super::export::{ExportInst, ExternVal};
use super::value::Val;

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
                .iter()
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
            locals: Rc::new(vec![]),
        }));

        let mut vals = Vec::with_capacity(module.globals.len());
        for global in &module.globals {
            vals.push(execute_expression(&global.init, stack, store)?);
        }

        if !stack.last().map(|entry| entry.is_frame()).unwrap_or(false) {
            return Err(Trap);
        }

        let mut refs_refs = Vec::with_capacity(module.elems.len());
        for elem in &module.elems {
            let init = Self::resolve_element_segment_init(elem);

            let mut refs = Vec::with_capacity(init.len());

            for init_expr in init {
                let ref_instr = match execute_expression(&init_expr, stack, store)? {
                    Val::Ref(ref_instr) => ref_instr,
                    _ => {
                        // unreachable due to validation
                        return Err(Trap);
                    }
                };
                refs.push(ref_instr);
            }

            refs_refs.push(refs);
        }

        if stack.pop_frame().is_none() {
            return Err(Trap);
        }

        let globals = externals
            .iter()
            .map(|export_inst| export_inst.value.clone())
            .collect();

        let module_inst_rc = store.allocate_module(module, globals, vals, refs_refs)?;

        unimplemented!()
    }

    fn resolve_element_segment_init<'a>(elem: &ElementSegmentType) -> Vec<ExpressionType> {
        match elem {
            ElementSegmentType::Active0Expr(active0_expr) => active0_expr.init.clone(),
            ElementSegmentType::Active0Functions(active0_funcs) => active0_funcs
                .init
                .iter()
                .map(|func_idx| {
                    ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                })
                .collect(),
            ElementSegmentType::ActiveRef(active0_ref) => active0_ref.init.clone(),
            ElementSegmentType::DeclarativeRef(declarative_ref) => declarative_ref.init.clone(),
            ElementSegmentType::ElemKindActiveFunctions(elemkind_active_funcs) => {
                elemkind_active_funcs
                    .init
                    .iter()
                    .map(|func_idx| {
                        ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                    })
                    .collect()
            }
            ElementSegmentType::ElemKindDeclarativeFunctions(elemkind_declarative_funcs) => {
                elemkind_declarative_funcs
                    .init
                    .iter()
                    .map(|func_idx| {
                        ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                    })
                    .collect()
            }
            ElementSegmentType::ElemKindPassiveFunctions(elemkind_passive_funcs) => {
                elemkind_passive_funcs
                    .init
                    .iter()
                    .map(|func_idx| {
                        ExpressionType::new(vec![InstructionType::RefFunc(func_idx.clone())])
                    })
                    .collect()
            }
            ElementSegmentType::PassiveRef(passive_ref) => passive_ref.init.clone(),
        }
    }
}
