mod context;

mod instructions;
pub mod types_validation;
pub mod validate_instruction;
mod validation_error;
mod validation_macros;
mod validation_stack;

// TODO: implement Module validation according to https://webassembly.github.io/spec/core/valid/modules.html#valid-module
