use syntax::types::LaneIdx;

use crate::result::{RResult, Trap};

use super::{
    stack::{Stack, StackEntry},
    value::Val,
};

pub fn vvunop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128) -> u128,
{
    if let Some(Val::Vec(vector)) = stack.pop_value() {
        stack.push_entry(StackEntry::Value(Val::Vec(operation(vector))));
    } else {
        return Err(Trap);
    }

    Ok(())
}

pub fn vternop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128, u128, u128) -> RResult<u128>,
{
    if let Some(Val::Vec(first)) = stack.pop_value() {
        if let Some(Val::Vec(second)) = stack.pop_value() {
            if let Some(Val::Vec(third)) = stack.pop_value() {
                stack.push_entry(StackEntry::Value(Val::Vec(operation(
                    first, second, third,
                )?)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    } else {
        return Err(Trap);
    }

    Ok(())
}

pub fn any_true(stack: &mut Stack) -> RResult<()> {
    if let Some(Val::Vec(vector)) = stack.pop_value() {
        let res = if vector != 0 { 1u32 } else { 0u32 };
        stack.push_entry(StackEntry::Value(Val::I32(res)));
    } else {
        return Err(Trap);
    }

    Ok(())
}

pub fn swizzle_i8x16(left: u128, right: u128) -> RResult<u128> {
    let left_lines = to_lanes_8x16(left);
    let right_lines = to_lanes_8x16(right);

    let mut new_lines = [0u8; 16];
    for (index, select_index) in right_lines.iter().enumerate() {
        new_lines[index] = *left_lines.get(*select_index as usize).unwrap_or(&0u8);
    }

    Ok(vec_from_lanes(new_lines.to_vec()))
}

pub fn shuffle_i8x16(left: u128, right: u128, lane_idx: &Vec<LaneIdx>) -> RResult<u128> {
    if lane_idx.iter().any(|LaneIdx(idx)| *idx > 32) {
        return Err(Trap);
    }
    let left_lanes = to_lanes_8x16(left);
    let right_lanes = to_lanes_8x16(right);
    let concatenated_lanes: Vec<&u8> = left_lanes.iter().chain(right_lanes.iter()).collect();

    let mut new_lines = [0u8; 16];
    for idx in 0..16 {
        let LaneIdx(i) = lane_idx[idx];
        new_lines[idx] = *concatenated_lanes[i as usize];
    }

    Ok(vec_from_lanes(new_lines.to_vec()))
}

pub fn to_lanes_8x16(vector: u128) -> [u8; 16] {
    vector.to_be_bytes()
}
pub fn from_lanes_8x16(lines: [u8; 16]) -> u128 {
    u128::from_be_bytes(lines)
}

pub fn vec_from_lanes<T>(lanes: Vec<T>) -> u128
where
    T: From<u8>
        + ::std::fmt::Binary
        + Into<u128>
        + ::std::marker::Copy
        + ::std::ops::BitAnd<Output = T>
        + ::std::ops::Shl<Output = T>,
{
    let mut result = 0u128;
    let bits_num = u128::BITS as usize / lanes.len();

    for lane in lanes.iter().rev() {
        let mut mask: T = 1u8.into();
        for _ in 0..bits_num {
            let bit = *lane & mask;
            result |= bit.into();
            mask = mask << 1u8.into();
        }
        result = result.rotate_right(bits_num as u32);
    }

    result
}

#[macro_export]
macro_rules! shape_splat_integer {
    ($stack: expr, $unpacked_type: path, $lane_type: ty, $shape_dim: expr) => {
        if let Some($unpacked_type(base)) = $stack.pop_value() {
            let mut lanes = Vec::with_capacity($shape_dim);
            for _ in 0..$shape_dim {
                lanes.push(base);
            }
            let vector = $crate::instances::instruction_vec::vec_from_lanes(lanes);
            $stack.push_entry(StackEntry::Value($crate::instances::value::Val::Vec(
                vector,
            )));
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! shape_splat_float {
    ($stack: expr, $unpacked_type: path, $lane_type: ty, $shape_dim: expr) => {
        if let Some($unpacked_type(base)) = $stack.pop_value() {
            let new_lane = <$lane_type>::from_be_bytes(base.to_be_bytes());
            let mut lanes = Vec::with_capacity($shape_dim);
            for _ in 0..$shape_dim {
                lanes.push(new_lane);
            }
            let vector = $crate::instances::instruction_vec::vec_from_lanes(lanes);
            $stack.push_entry(StackEntry::Value($crate::instances::value::Val::Vec(
                vector,
            )));
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! extract_lane_signed {
    ($stack: expr, $lane_type: ty, $res_type: path, $n: expr, $x: expr) => {{
        if !($x < $n) {
            return Err(Trap);
        }

        if let Some($crate::instances::value::Val::Vec(mut vector)) = $stack.pop_value() {
            let mut result: $lane_type = 0;
            let mut bit_mask: u128 = 1;
            vector = vector.rotate_right(($n as $lane_type) * ($x as $lane_type));
            let bits_number = 128 / $n;
            for _ in 0..bits_number {
                // copy bits
                let next_bit = vector & bit_mask;
                bit_mask = bit_mask.rotate_left(1);
                result |= next_bit as $lane_type;
            }
            todo!()
            // let signed_result = result as ;

            // $stack.push_entry(StackEntry::Value($res_type(result)))
        } else {
            return Err(Trap);
        }
    }};
}

#[macro_export]
macro_rules! extract_lane_unsigned {
    ($stack: expr, $lane_type: ty, $n: expr, $x: expr) => {};
}

#[macro_export]
macro_rules! extract_lane_float {
    ($stack: expr, $lane_type: ty, $n: expr, $x: expr) => {{
        if !($x < $n) {
            return Err(Trap);
        }

        if let Some($crate::instances::value::Val::Vec(vector)) = $stack.pop_value() {
            todo!();
        } else {
            return Err(Trap);
        }
    }};

    (float $stack: expr, $lane_type: ty, $n: expr, $x: expr) => {};
}
