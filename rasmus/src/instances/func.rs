use std::rc::Rc;

use super::module::ModuleInstance;
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
    pub module: Rc<ModuleInstance>,
    pub code: Func,
}

pub struct Func {
    pub func_type: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Vec<ExpressionType>,
}

pub struct HostFunc {
    pub func_type: FuncType,
    pub host_code: HostCode,
}

pub struct HostCode;
