use std::rc::Rc;

use super::module::ModuleInst;
use syntax::{
    instructions::ExpressionType,
    types::{FuncType, TypeIdx, ValType},
};

pub enum FuncInst {
    FuncInst(FuncInstLocal),
    HostFunc(HostFunc),
}

pub struct FuncInstLocal {
    pub func_type: FuncType,
    pub module: Rc<ModuleInst>,
    pub code: Func,
}

pub struct Func {
    pub func_type: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: ExpressionType,
}

pub struct HostFunc {
    pub func_type: FuncType,
    pub host_code: HostCode,
}

pub struct HostCode;