use crate::instances::export::ExportInst;
use syntax::module::Module;

pub struct ModuleRegistry;

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry
    }

    pub fn resolve_imports(&self, module: &Module) -> Vec<ExportInst> {
        unimplemented!()
    }
}
