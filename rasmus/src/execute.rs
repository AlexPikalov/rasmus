use std::ops::Neg;

use crate::result::{RResult, Trap};

use crate::instances::instruction::{
    iadd_32, iadd_64, iand, idiv_32_s, idiv_32_u, idiv_64_s, idiv_64_u, imul_32, imul_64, ior,
    irem_32_s, irem_32_u, irem_64_s, irem_64_u, isub_32, isub_64, ixor,
};
use crate::instances::stack::{Stack, StackEntry};
use crate::instances::store::Store;
use crate::instances::value::Val;
use crate::{binop, iextend, nearest};
use syntax::instructions::{ExpressionType, InstructionType};
use syntax::types::{Byte, F32Type, F64Type, I32Type, I64Type};

pub fn execute_expression(
    expr: &ExpressionType,
    stack: &mut Stack,
    store: &mut Store,
) -> RResult<Val> {
    // let frame = match stack.last().cloned() {
    //     Some(StackEntry::Frame(frame)) => frame,
    //     _ => return Err(Trap),
    // };

    for ref instr in &expr.instructions {
        execute_instruction(instr, stack, store)?;
    }

    stack.pop_value().ok_or(Trap)
}

pub fn execute_instruction(
    instr: &InstructionType,
    stack: &mut Stack,
    store: &mut Store,
    // frame_ref: &Frame,
) -> RResult<()> {
    match instr {
        InstructionType::I32Const(I32Type(num_val)) => {
            stack.push_entry(StackEntry::Value(Val::I32(*num_val)))
        }
        InstructionType::I64Const(I64Type(num_val)) => {
            stack.push_entry(StackEntry::Value(Val::I64(*num_val)))
        }
        InstructionType::F32Const(F32Type(num_val)) => {
            stack.push_entry(StackEntry::Value(Val::F32(*num_val)))
        }
        InstructionType::F64Const(F64Type(num_val)) => {
            stack.push_entry(StackEntry::Value(Val::F64(*num_val)))
        }
        InstructionType::V128Const(v128) => {
            stack.push_entry(StackEntry::Value(Val::Vec(i128_from_vec(v128)?)))
        }
        // iunop
        InstructionType::I32Clz => i32_unop(|v: u32| v.leading_zeros() as u32, stack)?,
        InstructionType::I64Clz => i64_unop(|v: u64| v.leading_zeros() as u64, stack)?,
        InstructionType::I32Ctz => i32_unop(|v: u32| v.trailing_zeros() as u32, stack)?,
        InstructionType::I64Ctz => i64_unop(|v: u64| v.trailing_zeros() as u64, stack)?,
        InstructionType::I32Popcnt => i32_unop(|v: u32| v.count_ones() as u32, stack)?,
        InstructionType::I64Popcnt => i64_unop(|v: u64| v.count_ones() as u64, stack)?,
        // funop
        InstructionType::F32Abs => f32_unop(|v: f32| v.abs(), stack)?,
        InstructionType::F64Abs => f64_unop(|v: f64| v.abs(), stack)?,
        InstructionType::F32Neg => f32_unop(|v: f32| v.neg(), stack)?,
        InstructionType::F64Neg => f64_unop(|v: f64| v.neg(), stack)?,
        InstructionType::F32Sqrt => f32_unop(|v: f32| v.sqrt(), stack)?,
        InstructionType::F64Sqrt => f64_unop(|v: f64| v.sqrt(), stack)?,
        InstructionType::F32Ceil => f32_unop(|v: f32| v.ceil(), stack)?,
        InstructionType::F64Ceil => f64_unop(|v: f64| v.ceil(), stack)?,
        InstructionType::F32Floor => f32_unop(|v: f32| v.floor(), stack)?,
        InstructionType::F64Floor => f64_unop(|v: f64| v.floor(), stack)?,
        InstructionType::F32Trunc => f32_unop(|v: f32| v.trunc(), stack)?,
        InstructionType::F64Trunc => f64_unop(|v: f64| v.trunc(), stack)?,
        InstructionType::F32Nearest => f32_unop(nearest!(f32), stack)?,
        InstructionType::F64Nearest => f64_unop(nearest!(f64), stack)?,
        // extendN_s
        InstructionType::I32Extend8S => i32_unop(iextend!(u32, 8usize), stack)?,
        InstructionType::I32Extend16S => i32_unop(iextend!(u32, 16usize), stack)?,
        InstructionType::I64Extend8S => i64_unop(iextend!(u64, 8usize), stack)?,
        InstructionType::I64Extend16S => i64_unop(iextend!(u64, 16usize), stack)?,
        InstructionType::I64Extend32S => i64_unop(iextend!(u64, 32usize), stack)?,
        // binop
        InstructionType::I32Add => {
            binop!(stack, Val::I32, Val::I32, Val::I32, iadd_32)
        }
        InstructionType::I64Add => {
            binop!(stack, Val::I64, Val::I64, Val::I64, iadd_64)
        }
        InstructionType::I32Sub => {
            binop!(stack, Val::I32, Val::I32, Val::I32, isub_32)
        }
        InstructionType::I64Sub => {
            binop!(stack, Val::I64, Val::I64, Val::I64, isub_64)
        }
        InstructionType::I32Mul => {
            binop!(stack, Val::I32, Val::I32, Val::I32, imul_32)
        }
        InstructionType::I64Mul => {
            binop!(stack, Val::I64, Val::I64, Val::I64, imul_64)
        }
        InstructionType::I32DivU => {
            binop!(stack, Val::I32, Val::I32, Val::I32, idiv_32_u)
        }
        InstructionType::I32DivS => {
            binop!(stack, Val::I32, Val::I32, Val::I32, idiv_32_s)
        }
        InstructionType::I64DivU => {
            binop!(stack, Val::I64, Val::I64, Val::I64, idiv_64_u)
        }
        InstructionType::I64DivS => {
            binop!(stack, Val::I64, Val::I64, Val::I64, idiv_64_s)
        }
        InstructionType::I32RemU => {
            binop!(stack, Val::I32, Val::I32, Val::I32, irem_32_u)
        }
        InstructionType::I32RemS => {
            binop!(stack, Val::I32, Val::I32, Val::I32, irem_32_s)
        }
        InstructionType::I64RemU => {
            binop!(stack, Val::I64, Val::I64, Val::I64, irem_64_u)
        }
        InstructionType::I64RemS => {
            binop!(stack, Val::I64, Val::I64, Val::I64, irem_64_s)
        }
        InstructionType::I32And => {
            binop!(stack, Val::I32, Val::I32, Val::I32, iand)
        }
        InstructionType::I64And => {
            binop!(stack, Val::I64, Val::I64, Val::I64, iand)
        }
        InstructionType::I32Or => {
            binop!(stack, Val::I32, Val::I32, Val::I32, ior)
        }
        InstructionType::I64Or => {
            binop!(stack, Val::I64, Val::I64, Val::I64, ior)
        }
        InstructionType::I32Xor => {
            binop!(stack, Val::I32, Val::I32, Val::I32, ixor)
        }
        InstructionType::I64Xor => {
            binop!(stack, Val::I64, Val::I64, Val::I64, ixor)
        } // _ => unimplemented!(),
    }

    Ok(())
}

fn i32_unop(exec_fn: impl FnOnce(u32) -> u32, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::I32(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::I32(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn i64_unop(exec_fn: impl FnOnce(u64) -> u64, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::I64(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::I64(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn f32_unop(exec_fn: impl FnOnce(f32) -> f32, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::F32(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::F32(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn f64_unop(exec_fn: impl FnOnce(f64) -> f64, stack: &mut Stack) -> RResult<()> {
    if let Some(Val::F64(v)) = stack.pop_value() {
        let result = exec_fn(v);
        stack.push_entry(StackEntry::Value(Val::F64(result)));
        return Ok(());
    } else {
        return Err(Trap);
    }
}

fn i128_from_vec(v: &Vec<Byte>) -> RResult<i128> {
    let slice: &[u8] = v.as_ref();
    let bytes: [u8; 16] = slice.try_into().map_err(|_| Trap)?;

    Ok(i128::from_le_bytes(bytes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instruction_i32_const() {
        let mut store = Store::new();
        let mut stack = Stack::new();
        assert!(execute_instruction(
            &InstructionType::I32Const(I32Type(1)),
            &mut stack,
            &mut store
        )
        .is_ok());
        if let Some(val) = stack.pop_value() {
            assert_eq!(val, Val::I32(1));
        } else {
            assert!(false, "stack should contain value");
        }
    }

    #[test]
    fn instruction_i64_const() {
        let mut store = Store::new();
        let mut stack = Stack::new();
        assert!(execute_instruction(
            &InstructionType::I64Const(I64Type(1)),
            &mut stack,
            &mut store
        )
        .is_ok());
        if let Some(val) = stack.pop_value() {
            assert_eq!(val, Val::I64(1));
        } else {
            assert!(false, "stack should contain value");
        }
    }

    #[test]
    fn instruction_f32_const() {
        let mut store = Store::new();
        let mut stack = Stack::new();
        assert!(execute_instruction(
            &InstructionType::F32Const(F32Type(1.0)),
            &mut stack,
            &mut store
        )
        .is_ok());
        if let Some(val) = stack.pop_value() {
            assert_eq!(val, Val::F32(1.0));
        } else {
            assert!(false, "stack should contain value");
        }
    }

    #[test]
    fn instruction_f64_const() {
        let mut store = Store::new();
        let mut stack = Stack::new();
        assert!(execute_instruction(
            &InstructionType::F64Const(F64Type(1.0)),
            &mut stack,
            &mut store
        )
        .is_ok());
        if let Some(val) = stack.pop_value() {
            assert_eq!(val, Val::F64(1.0));
        } else {
            assert!(false, "stack should contain value");
        }
    }

    #[test]
    fn instruction_v128_const() {
        let mut store = Store::new();
        let mut stack = Stack::new();
        let v128 = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let v128_as_num = i128_from_vec(&v128).unwrap();
        assert!(
            execute_instruction(&InstructionType::V128Const(v128), &mut stack, &mut store).is_ok()
        );
        if let Some(val) = stack.pop_value() {
            assert_eq!(val, Val::Vec(v128_as_num));
        } else {
            assert!(false, "stack should contain value");
        }
    }

    #[test]
    fn instruction_i32_add() {
        let mut store = Store::new();
        let mut stack = Stack::new();
        execute_instruction(
            &InstructionType::I32Const(I32Type(1)),
            &mut stack,
            &mut store,
        )
        .expect("should be able to put I32 const on stack");
        execute_instruction(
            &InstructionType::I32Const(I32Type(2)),
            &mut stack,
            &mut store,
        )
        .expect("should be able to put I32 const on stack");

        execute_instruction(&InstructionType::I32Add, &mut stack, &mut store)
            .expect("should be able to add two I32 numbers");
        if let Some(val) = stack.pop_value() {
            assert_eq!(val, Val::I32(3));
        } else {
            assert!(false, "stack should contain a proper value");
        }
    }
}
