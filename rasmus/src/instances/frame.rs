use std::rc::Rc;

use super::module::ModuleInst;
use super::value::Val;

#[derive(Debug)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub module: Rc<ModuleInst>,
}
