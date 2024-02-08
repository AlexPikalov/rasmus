use std::collections::VecDeque;

use crate::entities::{
    module::{DataType, Module},
    types::*,
};

#[derive(Debug, Clone, Default)]
pub struct ValidationContext {
    pub types: Vec<FuncType>,
    pub funcs: Vec<FuncType>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<GlobalType>,
    pub elems: Vec<RefType>,
    pub datas: Vec<DataType>,
    pub locals: Vec<ValType>,
    pub labels: VecDeque<ResultType>,
    pub maybe_return: Option<ResultType>,
    pub refs: Vec<FuncIdx>,
}

impl ValidationContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_prepended(&self, extend_with: &ValidationContext) -> Self {
        ValidationContext {
            types: {
                let mut types = vec![];
                types.extend(extend_with.types.iter().cloned());
                types.extend(self.types.iter().cloned());

                types
            },
            funcs: {
                let mut funcs = vec![];
                funcs.extend(extend_with.funcs.iter().cloned());
                funcs.extend(self.funcs.iter().cloned());

                funcs
            },
            tables: {
                let mut tables = vec![];
                tables.extend(extend_with.tables.iter().cloned());
                tables.extend(self.tables.iter().cloned());

                tables
            },
            mems: {
                let mut mems = vec![];
                mems.extend(extend_with.mems.iter().cloned());
                mems.extend(self.mems.iter().cloned());

                mems
            },
            globals: {
                let mut globals = vec![];
                globals.extend(extend_with.globals.iter().cloned());
                globals.extend(self.globals.iter().cloned());

                globals
            },
            elems: {
                let mut elems = vec![];
                elems.extend(extend_with.elems.iter().cloned());
                elems.extend(self.elems.iter().cloned());

                elems
            },
            datas: {
                let mut datas = vec![];
                datas.extend(extend_with.datas.iter().cloned());
                datas.extend(self.datas.iter().cloned());

                datas
            },
            locals: {
                let mut locals = vec![];
                locals.extend(extend_with.locals.iter().cloned());
                locals.extend(self.locals.iter().cloned());

                locals
            },
            labels: {
                let mut labels = VecDeque::new();
                labels.extend(extend_with.labels.iter().cloned());
                labels.extend(self.labels.iter().cloned());

                labels
            },
            maybe_return: {
                if extend_with.maybe_return.is_some() {
                    extend_with.maybe_return.clone()
                } else {
                    self.maybe_return.clone()
                }
            },
            refs: {
                let mut refs = vec![];
                refs.extend(extend_with.refs.iter().cloned());
                refs.extend(self.refs.iter().cloned());

                refs
            },
        }
    }
    pub fn get_extended(&self, extend_with: &ValidationContext) -> Self {
        ValidationContext {
            types: {
                let mut types = vec![];
                types.extend(self.types.iter().cloned());
                types.extend(extend_with.types.iter().cloned());

                types
            },
            funcs: {
                let mut funcs = vec![];
                funcs.extend(self.funcs.iter().cloned());
                funcs.extend(extend_with.funcs.iter().cloned());

                funcs
            },
            tables: {
                let mut tables = vec![];
                tables.extend(self.tables.iter().cloned());
                tables.extend(extend_with.tables.iter().cloned());

                tables
            },
            mems: {
                let mut mems = vec![];
                mems.extend(self.mems.iter().cloned());
                mems.extend(extend_with.mems.iter().cloned());

                mems
            },
            globals: {
                let mut globals = vec![];
                globals.extend(self.globals.iter().cloned());
                globals.extend(extend_with.globals.iter().cloned());

                globals
            },
            elems: {
                let mut elems = vec![];
                elems.extend(self.elems.iter().cloned());
                elems.extend(extend_with.elems.iter().cloned());

                elems
            },
            datas: {
                let mut datas = vec![];
                datas.extend(self.datas.iter().cloned());
                datas.extend(extend_with.datas.iter().cloned());

                datas
            },
            locals: {
                let mut locals = vec![];
                locals.extend(self.locals.iter().cloned());
                locals.extend(extend_with.locals.iter().cloned());

                locals
            },
            labels: {
                let mut labels = VecDeque::new();
                labels.extend(self.labels.iter().cloned());
                labels.extend(extend_with.labels.iter().cloned());

                labels
            },
            maybe_return: {
                if extend_with.maybe_return.is_some() {
                    extend_with.maybe_return.clone()
                } else {
                    self.maybe_return.clone()
                }
            },
            refs: {
                let mut refs = vec![];
                refs.extend(self.refs.iter().cloned());
                refs.extend(extend_with.refs.iter().cloned());

                refs
            },
        }
    }
}

impl From<Module> for ValidationContext {
    fn from(module: Module) -> Self {
        ValidationContext {
            types: module.types.clone(),
            funcs: module
                .funcs
                .iter()
                .map(|idx| module.types[idx.0 .0 as usize].clone())
                .collect(),
            tables: module.tables.clone(),
            mems: module.mems.clone(),
            globals: module
                .globals
                .iter()
                .map(|g| g.global_type.clone())
                .collect(),
            elems: module.elems.iter().map(|e| e.get_type()).collect(),
            datas: module.datas.clone(),
            locals: vec![],
            labels: VecDeque::new(),
            maybe_return: None,
            refs: vec![],
        }
    }
}
