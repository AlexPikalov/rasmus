use crate::types::*;

pub struct ValidationContext {
    pub types: Vec<FuncType>,
    pub funcs: Vec<FuncType>,
    pub tables: Vec<TableType>,
    pub mems: Vec<MemType>,
    pub globals: Vec<GlobalType>,
    pub elems: Vec<RefType>,
    // pub datas: Vec<ValidationResult::Ok> -- always valid
    pub locals: Vec<ValType>,
    pub labels: Vec<ResultType>,
    pub maybe_return: Option<ResultType>,
    pub refs: Vec<FuncIdx>,
}
