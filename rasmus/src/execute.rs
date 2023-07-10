use std::ops::Neg;

use crate::instances::instruction_vec::{shuffle_i8x16, swizzle_i8x16, vternop, vvunop};
use crate::result::{RResult, Trap};

use crate::instances::instruction::{
    bitselect, iadd_32, iadd_64, iand, iandnot, idiv_32_s, idiv_32_u, idiv_64_s, idiv_64_u,
    imul_32, imul_64, ior, irem_32_s, irem_32_u, irem_64_s, irem_64_u, irotl_32, irotl_64,
    irotr_32, irotr_64, is_ref_null, ishl_32, ishl_64, ishr_s_32, ishr_s_64, ishr_u_32, ishr_u_64,
    isub_32, isub_64, ixor, ref_func,
};
use crate::instances::ref_inst::RefInst;
use crate::instances::stack::{Stack, StackEntry};
use crate::instances::store::Store;
use crate::instances::value::Val;
use crate::{
    binop, binop_with_value, cvtop, demote, extract_lane_signed, float_s, float_u, iextend_s,
    nearest, promote, reinterpret, relop, shape_splat_float, shape_splat_integer, testop, trunc_s,
    trunc_sat_s, trunc_sat_u, trunc_u,
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
        InstructionType::I32Add => {
            binop!(stack, Val::I32, iadd_32)
        }
        InstructionType::I64Add => {
            binop!(stack, Val::I64, iadd_64)
        }
        InstructionType::I32Sub => {
            binop!(stack, Val::I32, isub_32)
        }
        InstructionType::I64Sub => {
            binop!(stack, Val::I64, isub_64)
        }
        InstructionType::I32Mul => {
            binop!(stack, Val::I32, imul_32)
        }
        InstructionType::I64Mul => {
            binop!(stack, Val::I64, imul_64)
        }
        InstructionType::I32DivU => {
            binop!(stack, Val::I32, idiv_32_u)
        }
        InstructionType::I32DivS => {
            binop!(stack, Val::I32, idiv_32_s)
        }
        InstructionType::I64DivU => {
            binop!(stack, Val::I64, idiv_64_u)
        }
        InstructionType::I64DivS => {
            binop!(stack, Val::I64, idiv_64_s)
        }
        InstructionType::I32RemU => {
            binop!(stack, Val::I32, irem_32_u)
        }
        InstructionType::I32RemS => {
            binop!(stack, Val::I32, irem_32_s)
        }
        InstructionType::I64RemU => {
            binop!(stack, Val::I64, irem_64_u)
        }
        InstructionType::I64RemS => {
            binop!(stack, Val::I64, irem_64_s)
        }
        InstructionType::I32And => {
            binop!(stack, Val::I32, iand)
        }
        InstructionType::I64And => {
            binop!(stack, Val::I64, iand)
        }
        InstructionType::I32Or => {
            binop!(stack, Val::I32, ior)
        }
        InstructionType::I64Or => {
            binop!(stack, Val::I64, ior)
        }
        InstructionType::I32Xor => {
            binop!(stack, Val::I32, ixor)
        }
        InstructionType::I64Xor => {
            binop!(stack, Val::I64, ixor)
        }
        InstructionType::I32Shl => {
            binop!(stack, Val::I32, ishl_32)
        }
        InstructionType::I64Shl => {
            binop!(stack, Val::I64, ishl_64)
        }
        InstructionType::I32ShrU => {
            binop!(stack, Val::I32, ishr_u_32)
        }
        InstructionType::I64ShrU => {
            binop!(stack, Val::I64, ishr_u_64)
        }
        InstructionType::I32ShrS => {
            binop!(stack, Val::I32, ishr_s_32)
        }
        InstructionType::I64ShrS => {
            binop!(stack, Val::I64, ishr_s_64)
        }
        InstructionType::I32Rotl => {
            binop!(stack, Val::I32, irotl_32)
        }
        InstructionType::I64Rotl => {
            binop!(stack, Val::I64, irotl_64)
        }
        InstructionType::I32Rotr => {
            binop!(stack, Val::I32, irotr_32)
        }
        InstructionType::I64Rotr => {
            binop!(stack, Val::I64, irotr_64)
        }
        // fbinop
        InstructionType::F32Add => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| Ok(lhs + rhs))
        }
        InstructionType::F64Add => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| Ok(lhs + rhs))
        }
        InstructionType::F32Sub => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| Ok(lhs - rhs))
        }
        InstructionType::F64Sub => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| Ok(lhs - rhs))
        }
        InstructionType::F32Mul => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| Ok(lhs * rhs))
        }
        InstructionType::F64Mul => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| Ok(lhs * rhs))
        }
        InstructionType::F32Div => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| Ok(lhs / rhs))
        }
        InstructionType::F64Div => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| Ok(lhs / rhs))
        }
        InstructionType::F32Min => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                if lhs == f32::NAN || rhs == f32::NAN {
                    return Ok(f32::NAN);
                }
                Ok(lhs.min(rhs))
            })
        }
        InstructionType::F64Min => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                if lhs == f64::NAN || rhs == f64::NAN {
                    return Ok(f64::NAN);
                }
                Ok(lhs.min(rhs))
            })
        }
        InstructionType::F32Max => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                if lhs == f32::NAN || rhs == f32::NAN {
                    return Ok(f32::NAN);
                }
                Ok(lhs.max(rhs))
            })
        }
        InstructionType::F64Max => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                if lhs == f64::NAN || rhs == f64::NAN {
                    return Ok(f64::NAN);
                }
                Ok(lhs.max(rhs))
            })
        }
        InstructionType::F32Copysign => {
            binop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(lhs.copysign(rhs))
            })
        }
        InstructionType::F64Copysign => {
            binop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(lhs.copysign(rhs))
            })
        }
        // testop
        InstructionType::I32Eqz => {
            testop!(stack, Val::I32, |val: u32| {
                Ok(if val == 0 { 1 } else { 0 })
            })
        }
        InstructionType::I64Eqz => {
            testop!(stack, Val::I64, |val: u64| {
                Ok(if val == 0 { 1 } else { 0 })
            })
        }
        // relop
        InstructionType::I32Eq => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if lhs == rhs { 1 } else { 0 })
            })
        }
        InstructionType::I64Eq => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if lhs == rhs { 1 } else { 0 })
            })
        }
        // relop
        InstructionType::I32Ne => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if lhs != rhs { 1 } else { 0 })
            })
        }
        InstructionType::I64Ne => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if lhs != rhs { 1 } else { 0 })
            })
        }
        InstructionType::I32LtS => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if (lhs as i32) < (rhs as i32) { 1 } else { 0 })
            })
        }
        InstructionType::I64LtS => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if (lhs as i64) < (rhs as i64) { 1 } else { 0 })
            })
        }
        InstructionType::I32LtU => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if lhs < rhs { 1 } else { 0 })
            })
        }
        InstructionType::I64LtU => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if lhs < rhs { 1 } else { 0 })
            })
        }
        InstructionType::I32GtS => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if (lhs as i32) > (rhs as i32) { 1 } else { 0 })
            })
        }
        InstructionType::I64GtS => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if (lhs as i64) > (rhs as i64) { 1 } else { 0 })
            })
        }
        InstructionType::I32GtU => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if lhs > rhs { 1 } else { 0 })
            })
        }
        InstructionType::I64GtU => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if lhs > rhs { 1 } else { 0 })
            })
        }
        InstructionType::I32LeS => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if (lhs as i32) <= (rhs as i32) { 1 } else { 0 })
            })
        }
        InstructionType::I64LeS => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if (lhs as i64) <= (rhs as i64) { 1 } else { 0 })
            })
        }
        InstructionType::I32LeU => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if lhs <= rhs { 1 } else { 0 })
            })
        }
        InstructionType::I64LeU => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if lhs <= rhs { 1 } else { 0 })
            })
        }
        InstructionType::I32GeS => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if (lhs as i32) >= (rhs as i32) { 1 } else { 0 })
            })
        }
        InstructionType::I64GeS => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if (lhs as i64) >= (rhs as i64) { 1 } else { 0 })
            })
        }
        InstructionType::I32GeU => {
            relop!(stack, Val::I32, |lhs: u32, rhs: u32| {
                Ok(if lhs >= rhs { 1 } else { 0 })
            })
        }
        InstructionType::I64GeU => {
            relop!(stack, Val::I64, |lhs: u64, rhs: u64| {
                Ok(if lhs >= rhs { 1 } else { 0 })
            })
        }
        InstructionType::F32Eq => {
            relop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(if lhs == rhs { 1 } else { 0 })
            })
        }
        InstructionType::F64Eq => {
            relop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(if lhs == rhs { 1 } else { 0 })
            })
        }
        InstructionType::F32Ne => {
            relop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(if lhs != rhs { 1 } else { 0 })
            })
        }
        InstructionType::F64Ne => {
            relop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(if lhs != rhs { 1 } else { 0 })
            })
        }
        InstructionType::F32Lt => {
            relop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(if lhs < rhs { 1 } else { 0 })
            })
        }
        InstructionType::F64Lt => {
            relop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(if lhs < rhs { 1 } else { 0 })
            })
        }
        InstructionType::F32Gt => {
            relop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(if lhs > rhs { 1 } else { 0 })
            })
        }
        InstructionType::F64Gt => {
            relop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(if lhs > rhs { 1 } else { 0 })
            })
        }
        InstructionType::F32Le => {
            relop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(if lhs <= rhs { 1 } else { 0 })
            })
        }
        InstructionType::F64Le => {
            relop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(if lhs <= rhs { 1 } else { 0 })
            })
        }
        InstructionType::F32Ge => {
            relop!(stack, Val::F32, |lhs: f32, rhs: f32| {
                Ok(if lhs >= rhs { 1 } else { 0 })
            })
        }
        InstructionType::F64Ge => {
            relop!(stack, Val::F64, |lhs: f64, rhs: f64| {
                Ok(if lhs >= rhs { 1 } else { 0 })
            })
        }
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

pub(super) fn v128_from_vec(v: &Vec<Byte>) -> RResult<u128> {
    let slice: &[u8] = v.as_ref();
    let bytes: [u8; 16] = slice.try_into().map_err(|_| Trap)?;

    Ok(u128::from_le_bytes(bytes))
}
