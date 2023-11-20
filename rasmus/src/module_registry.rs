use crate::entities::module::Module;
use crate::instances::export::ExportInst;

pub struct ModuleRegistry;

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry
    }

    pub fn resolve_imports(&self, module: &Module) -> Vec<ExportInst> {
        // TODO: validation
        //Assert:
        // is valid with external types
        // classifying its imports.
        //
        // If the number
        // of imports is not equal to the number
        // of provided external values, then:
        //
        // Fail.
        // TODO:
        vec![]
    }
}
