use std::ops::Neg;

use crate::instances::instruction_vec::{shuffle_i8x16, swizzle_i8x16, vternop, vvunop};
use crate::result::{RResult, Trap};

use crate::instances::instruction::{
    bitselect, eq, eqz, fadd, fdiv, fmul, fsub, ges, geu, gts, gtu, iadd_32, iadd_64, iand,
    iandnot, idiv_32_s, idiv_32_u, idiv_64_s, idiv_64_u, imul_32, imul_64, ior, irem_32_s,
    irem_32_u, irem_64_s, irem_64_u, irotl_32, irotl_64, irotr_32, irotr_64, is_ref_null, ishl_32,
    ishl_64, ishr_s_32, ishr_s_64, ishr_u_32, ishr_u_64, isub_32, isub_64, ixor, les, leu, lts,
    ltu, max, min, neq, ref_func,
};
use crate::instances::ref_inst::RefInst;
use crate::instances::stack::{Stack, StackEntry};
use crate::instances::store::Store;
use crate::instances::value::Val;
use crate::{
    binop, binop_impl, binop_with_value, cvtop, demote, extract_lane_signed, fcopysign, float_s,
    float_u, fmax, fmin, iextend_s, nearest, promote, reinterpret, relop, relop_impl,
    shape_splat_float, shape_splat_integer, testop_impl, trunc_s, trunc_sat_s, trunc_sat_u,
    trunc_u,
};
use syntax::instructions::{ExpressionType, InstructionType};
use syntax::types::{Byte, F32Type, F64Type, FuncIdx, I32Type, I64Type, LaneIdx, U32Type};

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
            stack.push_entry(StackEntry::Value(Val::Vec(v128_from_vec(v128)?)))
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
        InstructionType::I32Extend8S => i32_unop(iextend_s!(u32, i8), stack)?,
        InstructionType::I32Extend16S => i32_unop(iextend_s!(u32, i16), stack)?,
        InstructionType::I64Extend8S => i64_unop(iextend_s!(u64, i8), stack)?,
        InstructionType::I64Extend16S => i64_unop(iextend_s!(u64, i16), stack)?,
        InstructionType::I64Extend32S => i64_unop(iextend_s!(u64, i32), stack)?,
        // binop
        InstructionType::I32Add => i32_binop(iadd_32, stack)?,
        InstructionType::I64Add => i64_binop(iadd_64, stack)?,
        InstructionType::I32Sub => i32_binop(isub_32, stack)?,
        InstructionType::I64Sub => i64_binop(isub_64, stack)?,
        InstructionType::I32Mul => i32_binop(imul_32, stack)?,
        InstructionType::I64Mul => i64_binop(imul_64, stack)?,
        InstructionType::I32DivU => i32_binop(idiv_32_u, stack)?,
        InstructionType::I32DivS => i32_binop(idiv_32_s, stack)?,
        InstructionType::I64DivU => i64_binop(idiv_64_u, stack)?,
        InstructionType::I64DivS => i64_binop(idiv_64_s, stack)?,
        InstructionType::I32RemU => i32_binop(irem_32_u, stack)?,
        InstructionType::I32RemS => i32_binop(irem_32_s, stack)?,
        InstructionType::I64RemU => i64_binop(irem_64_u, stack)?,
        InstructionType::I64RemS => i64_binop(irem_64_s, stack)?,
        InstructionType::I32And => i32_binop(iand, stack)?,
        InstructionType::I64And => i64_binop(iand, stack)?,
        InstructionType::I32Or => i32_binop(ior, stack)?,
        InstructionType::I64Or => i64_binop(ior, stack)?,
        InstructionType::I32Xor => i32_binop(ixor, stack)?,
        InstructionType::I64Xor => i64_binop(ixor, stack)?,
        InstructionType::I32Shl => i32_binop(ishl_32, stack)?,
        InstructionType::I64Shl => i64_binop(ishl_64, stack)?,
        InstructionType::I32ShrU => i32_binop(ishr_u_32, stack)?,
        InstructionType::I64ShrU => i64_binop(ishr_u_64, stack)?,
        InstructionType::I32ShrS => i32_binop(ishr_s_32, stack)?,
        InstructionType::I64ShrS => i64_binop(ishr_s_64, stack)?,
        InstructionType::I32Rotl => i32_binop(irotl_32, stack)?,
        InstructionType::I64Rotl => i64_binop(irotl_64, stack)?,
        InstructionType::I32Rotr => i32_binop(irotr_32, stack)?,
        InstructionType::I64Rotr => i64_binop(irotr_64, stack)?,
        // fbinop
        InstructionType::F32Add => f32_binop(fadd, stack)?,
        InstructionType::F64Add => f64_binop(fadd, stack)?,
        InstructionType::F32Sub => f32_binop(fsub, stack)?,
        InstructionType::F64Sub => f64_binop(fsub, stack)?,
        InstructionType::F32Mul => f32_binop(fmul, stack)?,
        InstructionType::F64Mul => f64_binop(fmul, stack)?,
        InstructionType::F32Div => f32_binop(fdiv, stack)?,
        InstructionType::F64Div => f64_binop(fdiv, stack)?,
        InstructionType::F32Min => f32_binop(min, stack)?,
        InstructionType::F64Min => f64_binop(min, stack)?,
        InstructionType::F32Max => f32_binop(max, stack)?,
        InstructionType::F64Max => f64_binop(max, stack)?,
        InstructionType::F32Copysign => f32_binop(fcopysign!(f32), stack)?,
        InstructionType::F64Copysign => f64_binop(fcopysign!(f64), stack)?,
        // testop
        InstructionType::I32Eqz => i32_testop(eqz, stack)?,
        InstructionType::I64Eqz => i64_testop(eqz, stack)?,
        // relop
        InstructionType::I32Eq => i32_relop(eq, stack)?,
        InstructionType::I64Eq => i64_relop(eq, stack)?,
        InstructionType::I32Ne => i32_relop(neq, stack)?,
        InstructionType::I64Ne => i64_relop(neq, stack)?,
        InstructionType::I32LtS => i32_relop(lts, stack)?,
        InstructionType::I64LtS => i64_relop(lts, stack)?,
        InstructionType::I32LtU => i32_relop(ltu, stack)?,
        InstructionType::I64LtU => i64_relop(ltu, stack)?,
        InstructionType::I32GtS => i32_relop(gts, stack)?,
        InstructionType::I64GtS => i64_relop(gts, stack)?,
        InstructionType::I32GtU => i32_relop(gtu, stack)?,
        InstructionType::I64GtU => i64_relop(gtu, stack)?,
        InstructionType::I32LeS => i32_relop(les, stack)?,
        InstructionType::I64LeS => i64_relop(les, stack)?,
        InstructionType::I32LeU => i32_relop(leu, stack)?,
        InstructionType::I64LeU => i64_relop(leu, stack)?,
        InstructionType::I32GeS => i32_relop(ges, stack)?,
        InstructionType::I64GeS => i64_relop(ges, stack)?,
        InstructionType::I32GeU => i32_relop(geu, stack)?,
        InstructionType::I64GeU => i64_relop(geu, stack)?,
        InstructionType::F32Eq => f32_relop(eq, stack)?,
        InstructionType::F64Eq => f64_relop(eq, stack)?,
        InstructionType::F32Ne => f32_relop(neq, stack)?,
        InstructionType::F64Ne => f64_relop(neq, stack)?,
        InstructionType::F32Lt => f32_relop(ltu, stack)?,
        InstructionType::F64Lt => f64_relop(ltu, stack)?,
        InstructionType::F32Gt => f32_relop(gtu, stack)?,
        InstructionType::F64Gt => f64_relop(gtu, stack)?,
        InstructionType::F32Le => f32_relop(leu, stack)?,
        InstructionType::F64Le => f64_relop(leu, stack)?,
        InstructionType::F32Ge => f32_relop(geu, stack)?,
        InstructionType::F64Ge => f64_relop(geu, stack)?,
        // cvtop
        InstructionType::I32WrapI64 => {
            cvtop!(stack, Val::I64, Val::I32, |arg: u64| {
                Ok((arg as u128).rem_euclid(2u128).pow(32) as u32)
            })
        }
        InstructionType::I32TruncF32U => {
            cvtop!(stack, Val::F32, Val::I32, trunc_u!(f32, u32))
        }
        InstructionType::I32TruncF64U => {
            cvtop!(stack, Val::F64, Val::I32, trunc_u!(f64, u32))
        }
        InstructionType::I32TruncF32S => {
            cvtop!(stack, Val::F32, Val::I32, trunc_s!(f32, i32, u32))
        }
        InstructionType::I32TruncF64S => {
            cvtop!(stack, Val::F64, Val::I32, trunc_s!(f64, i32, u32))
        }
        InstructionType::I64TruncF32U => {
            cvtop!(stack, Val::F32, Val::I64, trunc_u!(f32, u64))
        }
        InstructionType::I64TruncF64U => {
            cvtop!(stack, Val::F64, Val::I64, trunc_u!(f64, u64))
        }
        InstructionType::I64TruncF32S => {
            cvtop!(stack, Val::F32, Val::I64, trunc_s!(f32, i64, u64))
        }
        InstructionType::I64TruncF64S => {
            cvtop!(stack, Val::F64, Val::I64, trunc_s!(f64, i64, u64))
        }
        InstructionType::I32TruncSatF32U => {
            cvtop!(stack, Val::F32, Val::I32, trunc_sat_u!(f32, u32))
        }
        InstructionType::I32TruncSatF64U => {
            cvtop!(stack, Val::F64, Val::I32, trunc_sat_u!(f64, u32))
        }
        InstructionType::I32TruncSatF32S => {
            cvtop!(stack, Val::F32, Val::I32, trunc_sat_s!(f32, i32, u32))
        }
        InstructionType::I32TruncSatF64S => {
            cvtop!(stack, Val::F64, Val::I32, trunc_sat_s!(f64, i32, u32))
        }
        InstructionType::I64TruncSatF32U => {
            cvtop!(stack, Val::F32, Val::I64, trunc_sat_u!(f32, u64))
        }
        InstructionType::I64TruncSatF64U => {
            cvtop!(stack, Val::F64, Val::I64, trunc_sat_u!(f64, u64))
        }
        InstructionType::I64TruncSatF32S => {
            cvtop!(stack, Val::F32, Val::I64, trunc_sat_s!(f32, i64, u64))
        }
        InstructionType::I64TruncSatF64S => {
            cvtop!(stack, Val::F64, Val::I64, trunc_sat_s!(f64, i64, u64))
        }
        InstructionType::F32ConvertI32S => {
            cvtop!(stack, Val::F32, Val::I32, float_s!(f32, i32, u32))
        }
        InstructionType::F32ConvertI32U => {
            cvtop!(stack, Val::F32, Val::I32, float_u!(f32, u32))
        }
        InstructionType::F32ConvertI64S => {
            cvtop!(stack, Val::F32, Val::I64, float_s!(f32, i64, u64))
        }
        InstructionType::F32ConvertI64U => {
            cvtop!(stack, Val::F32, Val::I64, float_u!(f32, u64))
        }
        InstructionType::F64ConvertI32S => {
            cvtop!(stack, Val::F64, Val::I32, float_s!(f64, i32, u32))
        }
        InstructionType::F64ConvertI32U => {
            cvtop!(stack, Val::F64, Val::I32, float_u!(f64, u32))
        }
        InstructionType::F64ConvertI64S => {
            cvtop!(stack, Val::F64, Val::I64, float_s!(f64, i64, u64))
        }
        InstructionType::F64ConvertI64U => {
            cvtop!(stack, Val::F64, Val::I64, float_u!(f64, u64))
        }
        InstructionType::F32DemoteF64 => {
            cvtop!(stack, Val::F64, Val::F32, demote!(f64, f32))
        }
        InstructionType::F64PromoteF32 => {
            cvtop!(stack, Val::F32, Val::F64, promote!(f32, f64))
        }
        InstructionType::F32ReinterpretI32 => {
            cvtop!(stack, Val::F32, Val::I32, reinterpret!(f32, u32))
        }
        InstructionType::F64ReinterpretI64 => {
            cvtop!(stack, Val::F64, Val::I64, reinterpret!(f64, u64))
        }
        // reference instructions
        InstructionType::RefNull(ref_type) => {
            stack.push_entry(StackEntry::Value(Val::Ref(RefInst::Null(ref_type.clone()))))
        }
        InstructionType::RefIsNull => {
            is_ref_null(stack)?;
        }
        InstructionType::RefFunc(FuncIdx(U32Type(func_idx))) => {
            ref_func(stack, *func_idx as usize)?;
        }
        // vector instructions
        InstructionType::V128Not => {
            vvunop(stack, ::std::ops::Not::not)?;
        }
        InstructionType::V128And => {
            binop!(stack, Val::Vec, iand);
        }
        InstructionType::V128AndNot => {
            binop!(stack, Val::Vec, iandnot);
        }
        InstructionType::V128Or => {
            binop!(stack, Val::Vec, ior);
        }
        InstructionType::V128Xor => {
            binop!(stack, Val::Vec, ixor);
        }
        InstructionType::V128Bitselect => {
            vternop(stack, bitselect)?;
        }
        InstructionType::I8x16Swizzle => {
            binop!(stack, Val::Vec, swizzle_i8x16)
        }
        InstructionType::I8x16Shuffle(lane_idx) => {
            binop_with_value!(stack, Val::Vec, lane_idx, shuffle_i8x16)
        }
        InstructionType::I8x16Splat => {
            shape_splat_integer!(stack, Val::I32, u8, 16usize);
        }
        InstructionType::I32x4Splat => {
            shape_splat_integer!(stack, Val::I32, u32, 4usize);
        }
        InstructionType::I64x2Splat => {
            shape_splat_integer!(stack, Val::I64, u64, 2usize);
        }
        InstructionType::F32x4Splat => {
            shape_splat_float!(stack, Val::F32, u32, 4usize)
        }
        InstructionType::F64x2Splat => {
            shape_splat_float!(stack, Val::F64, u64, 2usize)
        }
        InstructionType::I8x16ExtractLaneS(LaneIdx(lane_idx)) => {
            extract_lane_signed!(stack, u32, Val::I32, 16u8, *lane_idx);
        }
        _ => unimplemented!(),
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

binop_impl!(i32_binop, Val::I32, u32);
binop_impl!(i64_binop, Val::I64, u64);
binop_impl!(f32_binop, Val::F32, f32);
binop_impl!(f64_binop, Val::F64, f64);

testop_impl!(i32_testop, Val::I32, u32);
testop_impl!(i64_testop, Val::I64, u64);

relop_impl!(i32_relop, Val::I32, u32);
relop_impl!(i64_relop, Val::I64, u64);
relop_impl!(f32_relop, Val::F32, f32);
relop_impl!(f64_relop, Val::F64, f64);

pub(super) fn v128_from_vec(v: &Vec<Byte>) -> RResult<u128> {
    let slice: &[u8] = v.as_ref();
    let bytes: [u8; 16] = slice.try_into().map_err(|_| Trap)?;

    Ok(u128::from_le_bytes(bytes))
}
