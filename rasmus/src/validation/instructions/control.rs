use crate::{
    entities::{
        instructions::{
            BlockInstructionType, BlockType, IfElseInstructionType, InstructionType,
            LoopInstructionType,
        },
        types::{FuncIdx, LabelIdx, RefType, TableIdx, TypeIdx, U32Type, ValType},
    },
    validation::{
        context::ValidationContext,
        validation_error::{ValidationError, ValidationResult},
        validation_stack::{label_types, VType, ValidationStack, ValidationType},
    },
};

pub fn unreachable_instr(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    val_stack.unreachable()
}

pub fn block(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    block_instruction_type: &BlockInstructionType,
    validate_instruction: impl Fn(
        &InstructionType,
        &ValidationContext,
        &mut ValidationStack,
    ) -> ValidationResult<()>,
) -> ValidationResult<()> {
    let (input_types, output_types): (Vec<ValType>, Vec<ValType>) =
        get_input_output_types(&block_instruction_type.blocktype, ctx)?;

    val_stack.pop_vals(&input_types.iter().map(Into::into).collect())?;
    val_stack.push_ctrl(
        InstructionType::Block(block_instruction_type.clone()),
        input_types,
        output_types,
        false,
    );

    for instruction in &block_instruction_type.instructions {
        validate_instruction(&instruction, ctx, val_stack)?;
    }

    end(val_stack)
}

pub fn loop_instr(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    loop_instruction_type: &LoopInstructionType,
    validate_instruction: impl Fn(
        &InstructionType,
        &ValidationContext,
        &mut ValidationStack,
    ) -> ValidationResult<()>,
) -> ValidationResult<()> {
    let (input_types, output_types): (Vec<ValType>, Vec<ValType>) =
        get_input_output_types(&loop_instruction_type.blocktype, ctx)?;

    val_stack.pop_vals(&input_types.iter().map(Into::into).collect())?;
    val_stack.push_ctrl(
        InstructionType::Loop(loop_instruction_type.clone()),
        input_types,
        output_types,
        false,
    );

    for instruction in &loop_instruction_type.instructions {
        validate_instruction(&instruction, ctx, val_stack)?;
    }

    end(val_stack)
}

pub fn if_else(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    ifelse_instruction_type: &IfElseInstructionType,
    validate_instruction: impl Fn(
        &InstructionType,
        &ValidationContext,
        &mut ValidationStack,
    ) -> ValidationResult<()>,
) -> ValidationResult<()> {
    let (input_types, output_types): (Vec<ValType>, Vec<ValType>) =
        get_input_output_types(&ifelse_instruction_type.blocktype, ctx)?;

    val_stack.pop_vals(&input_types.iter().map(Into::into).collect())?;

    // if instruction sequence validation
    val_stack.push_ctrl(
        InstructionType::IfElse(ifelse_instruction_type.clone()),
        input_types.clone(),
        output_types.clone(),
        false,
    );

    for instruction in &ifelse_instruction_type.if_instructions {
        validate_instruction(&instruction, ctx, val_stack)?;
    }

    end(val_stack)?;

    // else instruction sequence validation
    val_stack.push_ctrl(
        InstructionType::IfElse(ifelse_instruction_type.clone()),
        input_types,
        output_types,
        false,
    );

    for instruction in &ifelse_instruction_type.else_instructions {
        validate_instruction(&instruction, ctx, val_stack)?;
    }

    end(val_stack)
}

pub fn br(
    val_stack: &mut ValidationStack,
    &LabelIdx(U32Type(label_idx)): &LabelIdx,
) -> ValidationResult<()> {
    let ctrl_frame = val_stack
        .get_ctrl(label_idx as usize)
        .ok_or_else(|| ValidationError::InsufficientOperandStackForInstruction)?;

    val_stack.pop_vals(&label_types(ctrl_frame).clone())?;

    val_stack.unreachable()?;
    end(val_stack)
}

pub fn br_if(
    val_stack: &mut ValidationStack,
    &LabelIdx(U32Type(label_idx)): &LabelIdx,
) -> ValidationResult<()> {
    let ctrl_frame = val_stack
        .get_ctrl(label_idx as usize)
        .ok_or_else(|| ValidationError::InsufficientOperandStackForInstruction)?;

    let types = label_types(ctrl_frame).clone();
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.pop_vals(&types)?;
    val_stack.push_vals_2(types);

    val_stack.unreachable()?;
    end(val_stack)
}

pub fn br_table(
    val_stack: &mut ValidationStack,
    (frames, LabelIdx(U32Type(m))): &(Vec<LabelIdx>, LabelIdx),
) -> ValidationResult<()> {
    val_stack.pop_val_expect(ValidationType::i32())?;

    if val_stack.ctrl_len() < *m as usize {
        return Err(ValidationError::InsufficientOperandStackForInstruction);
    }

    let ctrl_frame = val_stack
        .get_ctrl(*m as usize)
        .ok_or_else(|| ValidationError::InsufficientOperandStackForInstruction)?;

    let types = label_types(ctrl_frame).clone();
    let arity = types.len();

    for ref frame in frames {
        let ctrl_frame = val_stack
            .get_ctrl(frame.0 .0 as usize)
            .ok_or_else(|| ValidationError::InsufficientOperandStackForInstruction)?;
        let label_types_val = label_types(ctrl_frame).clone();
        if label_types_val.len() != arity {
            return Err(ValidationError::NotConsistentArity);
        }
        val_stack.push_vals_2(label_types_val);
    }

    val_stack.pop_vals(&types)?;
    val_stack.unreachable()?;

    end(val_stack)
}

pub fn return_instr(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
) -> ValidationResult<()> {
    let return_type = ctx
        .maybe_return
        .clone()
        .ok_or_else(|| ValidationError::ReturnNotFoundInContext)?
        .0;

    val_stack.pop_vals(&return_type.iter().map(Into::into).collect())?;

    val_stack.unreachable()?;
    end(val_stack)
}

pub fn call(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    FuncIdx(U32Type(func_idx)): &FuncIdx,
) -> ValidationResult<()> {
    let idx_usize = *func_idx as usize;
    let func_type = ctx
        .funcs
        .get(idx_usize)
        .ok_or_else(|| ValidationError::FuncTypeNotFound {
            func_idx: idx_usize,
        })?;

    val_stack.pop_vals(&func_type.parameters.iter().map(Into::into).collect())?;
    val_stack.push_vals(func_type.results.iter().map(Into::into).collect());

    Ok(())
}

pub fn call_indirect(
    val_stack: &mut ValidationStack,
    ctx: &ValidationContext,
    (TableIdx(U32Type(table_idx)), TypeIdx(U32Type(type_idx))): &(TableIdx, TypeIdx),
) -> ValidationResult<()> {
    let table_idx_usize = *table_idx as usize;
    let type_idx_usize = *type_idx as usize;

    let ref table_type = ctx
        .tables
        .get(table_idx_usize)
        .ok_or_else(|| ValidationError::TableNotFound)?
        .element_ref_type;

    if table_type != &RefType::FuncRef {
        return Err(ValidationError::UnexpectedRefType {
            actual: table_type.clone(),
            expected: RefType::FuncRef,
        });
    }

    let func_type =
        ctx.types
            .get(type_idx_usize)
            .ok_or_else(|| ValidationError::FuncTypeNotFound {
                func_idx: type_idx_usize,
            })?;

    val_stack.pop_vals(&func_type.parameters.iter().map(Into::into).collect())?;
    val_stack.pop_val_expect(ValidationType::i32())?;
    val_stack.push_vals(func_type.results.iter().map(Into::into).collect());

    Ok(())
}

fn end(val_stack: &mut ValidationStack) -> ValidationResult<()> {
    let frame = val_stack.pop_ctrl()?;
    val_stack.push_vals_2(frame.end_types);

    Ok(())
}

fn get_input_output_types(
    blocktype: &BlockType,
    ctx: &ValidationContext,
) -> ValidationResult<(Vec<ValType>, Vec<ValType>)> {
    Ok(match blocktype {
        BlockType::Empty => (vec![], vec![]),
        BlockType::ValType(ref val_type) => (vec![], vec![val_type.clone()]),
        BlockType::TypeIndex(ref type_idx) => {
            let func = ctx
                .types
                .get(type_idx.0 as usize)
                .ok_or_else(|| ValidationError::TypeNotFound)?;

            (func.parameters.clone(), func.results.clone())
        }
    })
}