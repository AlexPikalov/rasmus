pub(crate) mod data;
pub(crate) mod elem;
pub(crate) mod export;
pub(crate) mod frame;
pub(crate) mod func;
pub(crate) mod global;
pub(crate) mod instruction;
pub(crate) mod instruction_vec;
pub(crate) mod label;
pub(crate) mod memory;
pub(crate) mod module;
pub(crate) mod ref_inst;
pub(crate) mod stack;
pub(crate) mod store;
pub(crate) mod table;
pub(crate) mod value;

#[cfg(test)]
mod instruction_vec_test;
