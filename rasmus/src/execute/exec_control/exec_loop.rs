use std::rc::Rc;

use crate::entities::{
    module::{BlockType, InstructionType, LoopInstructionType},
    types::{FuncType, S33Type},
};

use crate::{
    instances::{label::LabelInst, stack::Stack, store::Store},
    result::{RResult, Trap},
};

use super::utils::pop_values_original_order;

pub fn exec_loop(
    stack: &mut Stack,
    store: &mut Store,
    &LoopInstructionType {
        ref blocktype,
        ref instructions,
    }: &LoopInstructionType,
    execute_instruction_fn: impl FnOnce(&InstructionType, &mut Stack, &mut Store) -> RResult<()> + Copy,
) -> RResult<()> {
    let current_frame = stack.current_frame().ok_or(Trap)?;
    let expand_blocktype = match blocktype {
        &BlockType::Empty => FuncType {
            parameters: vec![],
            results: vec![],
        },
        &BlockType::ValType(ref val_type) => FuncType {
            parameters: vec![],
            results: vec![val_type.clone()],
        },
        &BlockType::TypeIndex(S33Type(idx)) => current_frame
            .module
            .borrow()
            .types
            .get(idx as usize)
            .cloned()
            .ok_or(Trap)?,
    };

    let label = LabelInst {
        arity: expand_blocktype.results.len(),
        instructions: Rc::new(vec![InstructionType::Loop(LoopInstructionType {
            blocktype: blocktype.clone(),
            instructions: instructions.clone(),
        })]),
    };

    stack.push_label(label);

    // block input values according to blocktype
    let block_values = pop_values_original_order(stack, expand_blocktype.parameters.len())?;
    for value in block_values {
        stack.push_value(value);
    }

    // instructions execution
    for ref instruction in instructions {
        execute_instruction_fn(instruction, stack, store)?;
    }

    // taking result values according to blocktype
    let result_values = pop_values_original_order(stack, expand_blocktype.results.len())?;

    // drop label
    stack.pop_label().ok_or(Trap)?;

    // put result values back on stack
    for value in result_values {
        stack.push_value(value);
    }

    Ok(())
}
