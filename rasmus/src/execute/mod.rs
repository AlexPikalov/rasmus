mod exec_binop;
mod exec_const;
mod exec_cvtop;
mod exec_unop;

use crate::execute::exec_binop::{iand, iandnot, ior, ixor};
use crate::instances::instruction_vec::{shuffle_i8x16, swizzle_i8x16, vternop, vvunop};
use crate::result::{RResult, Trap};

use crate::instances::instruction::{
    bitselect, eq, eqz, ges, geu, gts, gtu, is_ref_null, les, leu, lts, ltu, neq, ref_func,
};
use crate::instances::ref_inst::RefInst;
use crate::instances::stack::{Stack, StackEntry};
use crate::instances::store::Store;
use crate::instances::value::Val;
use crate::{
    binop, binop_with_value, cvtop, demote, extract_lane_signed, float_s, float_u, promote,
    reinterpret, relop_impl, shape_splat_float, shape_splat_integer, testop_impl, trunc_s,
    trunc_sat_s, trunc_sat_u, trunc_u,
};
use syntax::instructions::{ExpressionType, InstructionType};
use syntax::types::{Byte, F32Type, F64Type, FuncIdx, I32Type, I64Type, LaneIdx, U32Type};

use self::exec_binop::{
    f32_add, f32_copysign, f32_div, f32_max, f32_min, f32_mul, f32_sub, f64_add, f64_copysign,
    f64_div, f64_max, f64_min, f64_mul, f64_sub, i32_add, i32_and, i32_div_s, i32_div_u, i32_mul,
    i32_or, i32_rem_s, i32_rem_u, i32_rotl, i32_rotr, i32_shl, i32_shr_s, i32_shr_u, i32_sub,
    i32_xor, i64_add, i64_and, i64_div_s, i64_div_u, i64_mul, i64_or, i64_rem_s, i64_rem_u,
    i64_rotl, i64_rotr, i64_shl, i64_shr_s, i64_shr_u, i64_sub, i64_xor,
};
use self::exec_const::{f32_const, f64_const, i32_const, i64_const, v128_const};
use self::exec_cvtop::{
    f64_i32_cvtop, i32_trunc_f32_s, i32_trunc_f32_u, i32_trunc_f64_s, i32_trunc_f64_u,
    i32_wrap_i64, i64_trunc_f32_s, i64_trunc_f32_u, i64_trunc_f64_s, i64_trunc_f64_u,
};
use self::exec_unop::{
    f32_abs, f32_ceil, f32_floor, f32_nearest, f32_neg, f32_sqrt, f32_trunc, f64_abs, f64_ceil,
    f64_floor, f64_nearest, f64_neg, f64_sqrt, f64_trunc, i32_clz, i32_ctz, i32_extend_16s,
    i32_extend_8s, i32_popcnt, i64_clz, i64_ctz, i64_extend_16s, i64_extend_32s, i64_extend_8s,
    i64_popcnt,
};

#[allow(dead_code)]
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
        InstructionType::I32Const(I32Type(num_val)) => i32_const(num_val, stack)?,
        InstructionType::I64Const(I64Type(num_val)) => i64_const(num_val, stack)?,
        InstructionType::F32Const(F32Type(num_val)) => f32_const(num_val, stack)?,
        InstructionType::F64Const(F64Type(num_val)) => f64_const(num_val, stack)?,
        InstructionType::V128Const(v128) => v128_const(v128, stack)?,
        // iunop
        InstructionType::I32Clz => i32_clz(stack)?,
        InstructionType::I64Clz => i64_clz(stack)?,
        InstructionType::I32Ctz => i32_ctz(stack)?,
        InstructionType::I64Ctz => i64_ctz(stack)?,
        InstructionType::I32Popcnt => i32_popcnt(stack)?,
        InstructionType::I64Popcnt => i64_popcnt(stack)?,
        // funop
        InstructionType::F32Abs => f32_abs(stack)?,
        InstructionType::F64Abs => f64_abs(stack)?,
        InstructionType::F32Neg => f32_neg(stack)?,
        InstructionType::F64Neg => f64_neg(stack)?,
        InstructionType::F32Sqrt => f32_sqrt(stack)?,
        InstructionType::F64Sqrt => f64_sqrt(stack)?,
        InstructionType::F32Ceil => f32_ceil(stack)?,
        InstructionType::F64Ceil => f64_ceil(stack)?,
        InstructionType::F32Floor => f32_floor(stack)?,
        InstructionType::F64Floor => f64_floor(stack)?,
        InstructionType::F32Trunc => f32_trunc(stack)?,
        InstructionType::F64Trunc => f64_trunc(stack)?,
        InstructionType::F32Nearest => f32_nearest(stack)?,
        InstructionType::F64Nearest => f64_nearest(stack)?,
        // extendN_s
        InstructionType::I32Extend8S => i32_extend_8s(stack)?,
        InstructionType::I32Extend16S => i32_extend_16s(stack)?,
        InstructionType::I64Extend8S => i64_extend_8s(stack)?,
        InstructionType::I64Extend16S => i64_extend_16s(stack)?,
        InstructionType::I64Extend32S => i64_extend_32s(stack)?,
        // binop
        InstructionType::I32Add => i32_add(stack)?,
        InstructionType::I64Add => i64_add(stack)?,
        InstructionType::I32Sub => i32_sub(stack)?,
        InstructionType::I64Sub => i64_sub(stack)?,
        InstructionType::I32Mul => i32_mul(stack)?,
        InstructionType::I64Mul => i64_mul(stack)?,
        InstructionType::I32DivU => i32_div_u(stack)?,
        InstructionType::I32DivS => i32_div_s(stack)?,
        InstructionType::I64DivU => i64_div_u(stack)?,
        InstructionType::I64DivS => i64_div_s(stack)?,
        InstructionType::I32RemU => i32_rem_u(stack)?,
        InstructionType::I32RemS => i32_rem_s(stack)?,
        InstructionType::I64RemU => i64_rem_u(stack)?,
        InstructionType::I64RemS => i64_rem_s(stack)?,
        InstructionType::I32And => i32_and(stack)?,
        InstructionType::I64And => i64_and(stack)?,
        InstructionType::I32Or => i32_or(stack)?,
        InstructionType::I64Or => i64_or(stack)?,
        InstructionType::I32Xor => i32_xor(stack)?,
        InstructionType::I64Xor => i64_xor(stack)?,
        InstructionType::I32Shl => i32_shl(stack)?,
        InstructionType::I64Shl => i64_shl(stack)?,
        InstructionType::I32ShrU => i32_shr_u(stack)?,
        InstructionType::I64ShrU => i64_shr_u(stack)?,
        InstructionType::I32ShrS => i32_shr_s(stack)?,
        InstructionType::I64ShrS => i64_shr_s(stack)?,
        InstructionType::I32Rotl => i32_rotl(stack)?,
        InstructionType::I64Rotl => i64_rotl(stack)?,
        InstructionType::I32Rotr => i32_rotr(stack)?,
        InstructionType::I64Rotr => i64_rotr(stack)?,
        // fbinop
        InstructionType::F32Add => f32_add(stack)?,
        InstructionType::F64Add => f64_add(stack)?,
        InstructionType::F32Sub => f32_sub(stack)?,
        InstructionType::F64Sub => f64_sub(stack)?,
        InstructionType::F32Mul => f32_mul(stack)?,
        InstructionType::F64Mul => f64_mul(stack)?,
        InstructionType::F32Div => f32_div(stack)?,
        InstructionType::F64Div => f64_div(stack)?,
        InstructionType::F32Min => f32_min(stack)?,
        InstructionType::F64Min => f64_min(stack)?,
        InstructionType::F32Max => f32_max(stack)?,
        InstructionType::F64Max => f64_max(stack)?,
        InstructionType::F32Copysign => f32_copysign(stack)?,
        InstructionType::F64Copysign => f64_copysign(stack)?,
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
        InstructionType::I32WrapI64 => i32_wrap_i64(stack)?,
        InstructionType::I32TruncF32U => i32_trunc_f32_u(stack)?,
        InstructionType::I32TruncF64U => i32_trunc_f64_u(stack)?,
        InstructionType::I32TruncF32S => i32_trunc_f32_s(stack)?,
        InstructionType::I32TruncF64S => i32_trunc_f64_s(stack)?,
        InstructionType::I64TruncF32U => i64_trunc_f32_u(stack)?,
        InstructionType::I64TruncF64U => i64_trunc_f64_u(stack)?,
        InstructionType::I64TruncF32S => i64_trunc_f32_s(stack)?,
        InstructionType::I64TruncF64S => i64_trunc_f64_s(stack)?,
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
