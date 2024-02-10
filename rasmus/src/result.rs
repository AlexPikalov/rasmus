use crate::{
    binary::syntax_error::SyntaxError, instances::value::Val, module_registry::ModuleRegistryError,
};

pub type CompResult = Result<Val, Trap>;

pub type RResult<T> = Result<T, Trap>;

#[derive(Debug)]
pub struct Trap;

pub struct ErrorStack;

impl From<ModuleRegistryError> for Trap {
    fn from(_registry_error: ModuleRegistryError) -> Self {
        Trap
    }
}

impl From<SyntaxError> for Trap {
    fn from(_syntax_error: SyntaxError) -> Self {
        Trap
    }
}
