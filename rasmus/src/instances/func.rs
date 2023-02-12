use std::rc::Rc;

use super::module::ModuleInst;
use syntax::types::{Func, FuncType};

pub enum FuncInst {
    FuncInst(FuncInstLocal),
    HostFunc(HostFunc),
}

pub struct FuncInstLocal {
    pub func_type: FuncType,
    pub module: Rc<ModuleInst>,
    pub code: Func,
}

pub struct HostFunc {
    pub func_type: FuncType,
    pub host_code: HostCode,
}

pub struct HostCode;
