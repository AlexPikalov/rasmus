use crate::{
    instances::{
        stack::{Stack, StackEntry},
        value::Val,
    },
    result::{RResult, Trap},
};

use super::{to_lanes_32x4, vec_from_lanes};

pub fn i32x4_vcvtop_f32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(f32) -> u32 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    let mut new_lanes: Vec<u32> = Vec::with_capacity(4);

    for lane in lanes {
        let float = f32::from_be_bytes(lane.to_be_bytes());
        new_lanes.push(func(float));
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub fn shape_i32_trunc_f32_s(f: f32) -> u32 {
    if f.is_nan() {
        return 0;
    }

    if f.is_infinite() {
        if f.is_sign_positive() {
            return i32::MAX as u32;
        } else {
            return i32::MIN as u32;
        }
    }

    let trunced = f.trunc() as i128;

    if trunced > i32::MAX as i128 {
        return i32::MAX as u32;
    }

    if trunced < i32::MIN as i128 {
        return i32::MIN as u32;
    }

    trunced as u32
}

pub fn shape_i32_trunc_f32_u(f: f32) -> u32 {
    if f.is_nan() {
        return 0;
    }

    if f.is_infinite() {
        if f.is_sign_positive() {
            return u32::MAX;
        } else {
            return 0;
        }
    }

    let trunced = f.trunc() as u128;

    if trunced > u32::MAX as u128 {
        return u32::MAX;
    }

    if trunced < 0 {
        return 0;
    }

    trunced as u32
}

pub fn f32x4_vcvtop_i32x4<F>(stack: &mut Stack, func: F) -> RResult<()>
where
    F: FnOnce(u32) -> u32 + Copy,
{
    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    let mut new_lanes: Vec<u32> = Vec::with_capacity(4);

    for lane in lanes {
        new_lanes.push(func(lane));
    }

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(new_lanes))));

    Ok(())
}

pub fn shape_f32_convert_i32_u(v: u32) -> u32 {
    u32::from_be_bytes((v as f32).to_be_bytes())
}

pub fn shape_f32_convert_i32_s(v: u32) -> u32 {
    u32::from_be_bytes((v as i32 as f32).to_be_bytes())
}
