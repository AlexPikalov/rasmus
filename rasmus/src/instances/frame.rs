use std::cell::RefCell;
use std::rc::Rc;

use super::module::ModuleInst;
use super::value::Val;

#[derive(Debug, Clone)]
pub struct Frame {
    pub locals: Rc<Vec<Val>>,
    pub module: Rc<RefCell<ModuleInst>>,
}
