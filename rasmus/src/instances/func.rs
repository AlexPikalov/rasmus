use std::{cell::RefCell, rc::Rc};

use crate::{
    execute::pop_values_original_order,
    result::{RResult, Trap},
};

use super::{module::ModuleInst, stack::Stack, store::Store};
use crate::entities::{
    module::InstructionType,
    types::{Func, FuncType},
};

#[derive(Clone, Debug)]
pub enum FuncInst {
    FuncInst(FuncInstLocal),
    HostFunc(HostFunc),
}

impl FuncInst {
    pub fn get_type(&self) -> &FuncType {
        match self {
            FuncInst::FuncInst(f) => &f.func_type,
            FuncInst::HostFunc(h) => &h.func_type,
        }
    }

    pub fn get_module(&self) -> Rc<RefCell<ModuleInst>> {
        match self {
            FuncInst::FuncInst(f) => f.module.clone(),
            FuncInst::HostFunc(_) => todo!(),
        }
    }

    pub fn invoke(
        &self,
        stack: &mut Stack,
        store: &mut Store,
        execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<()>
            + Copy,
    ) -> RResult<()> {
        match self {
            FuncInst::FuncInst(f) => f.invoke(stack, store, execute_instruction_fn),
            FuncInst::HostFunc(h) => h.invoke(stack, store),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FuncInstLocal {
    pub func_type: FuncType,
    pub module: Rc<RefCell<ModuleInst>>,
    pub code: Func,
}

impl FuncInstLocal {
    pub fn invoke(
        &self,
        stack: &mut Stack,
        store: &mut Store,
        execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<()>
            + Copy,
    ) -> RResult<()> {
        let mut returned = false;
        for ref instruction in &self.code.body.instructions {
            // TODO: handle function finishing after return
            execute_instruction_fn(instruction, stack, store)?;
            if let InstructionType::Return = instruction {
                returned = true;
                break;
            }
        }

        if !returned {
            let result = pop_values_original_order(stack, self.func_type.results.len())?;
            stack.pop_label().ok_or(Trap)?;
            stack.pop_frame().ok_or(Trap)?;

            for value in result {
                stack.push_value(value);
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct HostFunc {
    pub func_type: FuncType,
    pub host_code: HostCode,
}

impl HostFunc {
    pub fn invoke(&self, stack: &mut Stack, store: &mut Store) -> RResult<()> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HostCode;
