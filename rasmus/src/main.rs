use std::fs::read;

use crate::{
    binary::module_parser::ModuleParser,
    controller::run_func,
    instances::{module::ModuleInst, stack::Stack, store::Store, value::Val},
    module_registry::ModuleRegistry,
};

mod address;
mod binary;
mod cli;
mod controller;
mod entities;
mod execute;
mod instances;
mod module;
mod module_registry;
mod result;
pub mod sign;
mod validation;

use binary::parse_trait::ParseBin;

#[cfg(test)]
mod execute_test;
#[cfg(test)]
mod test_utils;

fn main() {
    let file_content = read("./rasmus/tests/files/factorial.wasm").expect("should read");
    let (_, module) = ModuleParser::parse(&file_content).unwrap();

    let module_registry = Box::new(ModuleRegistry::new());
    let mut store = Store::new();
    let mut stack = Stack::new();
    let module_inst = ModuleInst::instantiate(&mut store, &mut stack, &module, &module_registry)
        .expect("should instantiate module");

    let result = run_func(
        module_inst.clone(),
        &module,
        "factorial",
        vec![Val::I32(10)],
        &mut stack,
        &mut store,
    )
    .expect("finish without errors");

    println!("result >>> {:?}", result);
}
