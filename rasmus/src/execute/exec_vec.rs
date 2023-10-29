use syntax::types::LaneIdx;

use crate::result::{RResult, Trap};

use crate::instances::{
    stack::{Stack, StackEntry},
    value::Val,
};

use super::exec_binop::{iand, iandnot, ior, ixor};

pub fn vbinop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128, u128) -> RResult<u128>,
{
    let first = stack.pop_v128().ok_or(Trap)?;
    let second = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(first, second)?)));

    Ok(())
}

pub fn vbinop_with_value<Op, V>(stack: &mut Stack, operation: Op, value: V) -> RResult<()>
where
    Op: Fn(u128, u128, V) -> RResult<u128>,
{
    let first = stack.pop_v128().ok_or(Trap)?;
    let second = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(
        first, second, value,
    )?)));

    Ok(())
}

pub fn vvunop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128) -> u128,
{
    let v128 = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(v128))));

    Ok(())
}

pub fn vternop<Op>(stack: &mut Stack, operation: Op) -> RResult<()>
where
    Op: Fn(u128, u128, u128) -> RResult<u128>,
{
    let first = stack.pop_v128().ok_or(Trap)?;
    let second = stack.pop_v128().ok_or(Trap)?;
    let third = stack.pop_v128().ok_or(Trap)?;
    stack.push_entry(StackEntry::Value(Val::Vec(operation(
        first, second, third,
    )?)));

    Ok(())
}

pub fn v128_and(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, iand)
}

pub fn v128_andnot(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, iandnot)
}

pub fn v128_or(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, ior)
}

pub fn v128_xor(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, ixor)
}

pub fn v128_anytrue(stack: &mut Stack) -> RResult<()> {
    if let Some(Val::Vec(vector)) = stack.pop_value() {
        let res = if vector != 0 { 1u32 } else { 0u32 };
        stack.push_entry(StackEntry::Value(Val::I32(res)));
    } else {
        return Err(Trap);
    }

    Ok(())
}

pub fn i8x16_swizzle(stack: &mut Stack) -> RResult<()> {
    vbinop(stack, inner_swizzle_i8x16)
}

fn inner_swizzle_i8x16(left: u128, right: u128) -> RResult<u128> {
    let left_lines = to_lanes_8x16(left);
    let right_lines = to_lanes_8x16(right);

    let mut new_lines = [0u8; 16];
    for (index, select_index) in right_lines.iter().enumerate() {
        new_lines[index] = *left_lines.get(*select_index as usize).unwrap_or(&0u8);
    }

    Ok(vec_from_lanes(new_lines.to_vec()))
}

pub fn i8x16_shuffle(stack: &mut Stack, value: &Vec<LaneIdx>) -> RResult<()> {
    vbinop_with_value(stack, inner_shuffle_i8x16, value)
}

fn inner_shuffle_i8x16(left: u128, right: u128, lane_idx: &Vec<LaneIdx>) -> RResult<u128> {
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

pub fn to_lanes_16x8(vector: u128) -> [u16; 8] {
    let lanes = to_lanes_8x16(vector);
    let mut arr = [0u16; 8];

    for i in 0..arr.len() {
        arr[i] = u16::from_be_bytes([lanes[i * 2], lanes[i * 2 + 1]]);
    }

    arr
}

pub fn to_lanes_32x4(vector: u128) -> [u32; 4] {
    let lanes = to_lanes_8x16(vector);
    let mut arr = [0u32; 4];

    for i in 0..arr.len() {
        arr[i] = u32::from_be_bytes([
            lanes[i * 4],
            lanes[i * 4 + 1],
            lanes[i * 4 + 2],
            lanes[i * 4 + 3],
        ]);
    }

    arr
}

pub fn to_lanes_64x2(vector: u128) -> [u64; 2] {
    let lanes = to_lanes_8x16(vector);
    let mut arr = [0u64; 2];

    for i in 0..arr.len() {
        arr[i] = u64::from_be_bytes([
            lanes[i * 8],
            lanes[i * 8 + 1],
            lanes[i * 8 + 2],
            lanes[i * 8 + 3],
            lanes[i * 8 + 4],
            lanes[i * 8 + 5],
            lanes[i * 8 + 6],
            lanes[i * 8 + 7],
        ]);
    }

    arr
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

pub fn i8x16_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)? as u8;
    let shape_dim = 16usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i16x8_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)? as u16;
    let shape_dim = 8usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i32x4_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)?;
    let shape_dim = 4usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i64x2_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i64().ok_or(Trap)?;
    let shape_dim = 2usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(base);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn f32x4_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i32().ok_or(Trap)?;
    let new_lane = u32::from_be_bytes(base.to_be_bytes());
    let shape_dim = 4usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(new_lane);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn f64x2_splat(stack: &mut Stack) -> RResult<()> {
    let base = stack.pop_i64().ok_or(Trap)?;
    let new_lane = u64::from_be_bytes(base.to_be_bytes());
    let shape_dim = 2usize;

    let mut lanes = Vec::with_capacity(shape_dim);
    for _ in 0..shape_dim {
        lanes.push(new_lane);
    }
    let vector = vec_from_lanes(lanes);
    stack.push_entry(StackEntry::Value(Val::Vec(vector)));

    Ok(())
}

pub fn i8x16_extract_lane_s(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 16usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as i8 as u32)));

    Ok(())
}

pub fn i8x16_extract_lane_u(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 16usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_8x16(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as u32)));

    Ok(())
}

pub fn i16x8_extract_lane_s(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as i16 as u32)));

    Ok(())
}

pub fn i16x8_extract_lane_u(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_16x8(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i] as u32)));

    Ok(())
}

pub fn i32x4_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 4usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    stack.push_entry(StackEntry::Value(Val::I32(lanes[lane_i])));

    Ok(())
}

pub fn i64x2_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_64x2(vector);

    stack.push_entry(StackEntry::Value(Val::I64(lanes[lane_i])));

    Ok(())
}

pub fn f32x4_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 4usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_32x4(vector);

    let float = f32::from_be_bytes(lanes[lane_i].to_be_bytes());
    stack.push_entry(StackEntry::Value(Val::F32(float)));

    Ok(())
}

pub fn f64x2_extract_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let vector = stack.pop_v128().ok_or(Trap)?;
    let lanes = to_lanes_64x2(vector);

    let float = f64::from_be_bytes(lanes[lane_i].to_be_bytes());
    stack.push_entry(StackEntry::Value(Val::F64(float)));

    Ok(())
}

pub fn i8x16_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 16usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i32().ok_or(Trap)? as u8;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_8x16(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn i16x8_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i32().ok_or(Trap)? as u16;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_16x8(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn i32x4_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 4usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i32().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_32x4(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn i64x2_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = stack.pop_i64().ok_or(Trap)?;
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_64x2(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn f32x4_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 8usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = u32::from_be_bytes(stack.pop_f32().ok_or(Trap)?.to_be_bytes());
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_32x4(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

pub fn f64x2_replace_lane(stack: &mut Stack, lane_idx: u8) -> RResult<()> {
    let shape_dim = 2usize;
    let lane_i = lane_idx as usize;
    if !(lane_i < shape_dim) {
        return Err(Trap);
    }

    let new_lane = u64::from_be_bytes(stack.pop_f64().ok_or(Trap)?.to_be_bytes());
    let vector = stack.pop_v128().ok_or(Trap)?;
    let mut lanes = to_lanes_64x2(vector);
    lanes[lane_i] = new_lane;

    stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(lanes.to_vec()))));

    Ok(())
}

#[cfg(test)]
mod test {
    use syntax::types::LaneIdx;

    use super::*;

    #[test]
    fn vec_from_lanes_test() {
        // i8x16
        assert_eq!(
            vec_from_lanes(vec![
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],),
            0
        );

        assert_eq!(
        vec_from_lanes(vec![
            0b00010001u8,
            0b00010001u8,
            0b00010001u8,
            0b00010001u8,
            0b00010001u8,
            0b00010001u8,
            0b00010001u8,
            0b00010001u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
        ],),
        0b00010001000100010001000100010001000100010001000100010001000100010000000000000000000000000000000000000000000000000000000000000000u128
    );

        assert_eq!(
            vec_from_lanes(vec![
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
                u8::MAX,
            ],),
            u128::MAX
        );

        // i16x8
        assert_eq!(
            vec_from_lanes(vec![0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16]),
            0,
        );

        assert_eq!(
            vec_from_lanes(vec![
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
                u16::MAX,
            ]),
            u128::MAX,
        );

        // i32x4
        assert_eq!(vec_from_lanes(vec![0u32, 0u32, 0u32, 0u32,]), 0);
        assert_eq!(
            vec_from_lanes(vec![u32::MAX, u32::MAX, u32::MAX, u32::MAX,]),
            u128::MAX
        );

        // i64x2
        assert_eq!(vec_from_lanes(vec![0u64, 0u64]), 0);
        assert_eq!(vec_from_lanes(vec![u64::MAX, u64::MAX]), u128::MAX);
    }

    #[test]
    fn vec_to_lanes_16x8() {
        let init_lanes = vec![1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16];
        let vector = vec_from_lanes(init_lanes.clone());

        let result_lanes = to_lanes_16x8(vector);

        assert_eq!(
            result_lanes.to_vec(),
            init_lanes,
            "should properly return 16x8 lanes"
        );
    }

    #[test]
    fn vec_to_lanes_32x4() {
        let init_lanes = vec![1u32, 2u32, 3u32, 4u32];
        let vector = vec_from_lanes(init_lanes.clone());

        let result_lanes = to_lanes_32x4(vector);

        assert_eq!(
            result_lanes.to_vec(),
            init_lanes,
            "should properly return 16x8 lanes"
        );
    }

    #[test]
    fn vec_to_lanes_64x2() {
        let init_lanes = vec![1u64, 2u64];
        let vector = vec_from_lanes(init_lanes.clone());

        let result_lanes = to_lanes_64x2(vector);

        assert_eq!(
            result_lanes.to_vec(),
            init_lanes,
            "should properly return 16x8 lanes"
        );
    }

    #[test]
    fn swizzle_i8x16_test() {
        let src = vec_from_lanes(vec![
            1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);
        let pick = vec_from_lanes(vec![0u8, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15]);
        let expected = vec_from_lanes(vec![
            1u8, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16,
        ]);
        assert_eq!(
            inner_swizzle_i8x16(src, pick).expect("should swizzle without errors"),
            expected
        );
    }

    #[test]
    fn shuffle_i8x16_test() {
        let left = vec_from_lanes(vec![
            1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);
        let right = vec_from_lanes(vec![
            17u8, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        ]);

        assert!(
            inner_shuffle_i8x16(
                left,
                right,
                &([33u8; 16].iter().map(|v| LaneIdx(*v)).collect())
            )
            .is_err(),
            "should return error if any of lane_idx is more or equal 32"
        );

        assert_eq!(
            inner_shuffle_i8x16(
                left,
                right,
                &(vec![0u8, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32]
                    .iter()
                    .map(|v| LaneIdx(*v))
                    .collect())
            )
            .expect("should shuffle without errors"),
            vec_from_lanes(vec![
                1u8, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31
            ]),
            "should return error if any of lane_idx is more or equal 32"
        );
    }

    #[test]
    fn shape_splat_i8x16_test() {
        let mut stack = Stack::new();
        stack.push_entry(StackEntry::Value(Val::I32(1)));
        i8x16_splat(&mut stack).expect("should splat without errors");
        let val = stack.pop_value();
        let expected = vec_from_lanes(vec![1u8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
    }

    #[test]
    fn shape_splat_i16x8_test() {
        let mut stack = Stack::new();
        stack.push_entry(StackEntry::Value(Val::I32(1)));
        i16x8_splat(&mut stack).expect("should splat without errors");
        let val = stack.pop_value();
        let expected = vec_from_lanes(vec![1u16, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
    }

    #[test]
    fn shape_splat_i32x4_test() {
        let mut stack = Stack::new();
        stack.push_entry(StackEntry::Value(Val::I32(1)));
        i32x4_splat(&mut stack).expect("should splat without errors");
        let val = stack.pop_value();
        let expected = vec_from_lanes(vec![1u32, 1u32, 1u32, 1u32]);
        assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
    }

    #[test]
    fn shape_splat_i64x2_test() {
        let mut stack = Stack::new();
        stack.push_entry(StackEntry::Value(Val::I64(2)));
        i64x2_splat(&mut stack).expect("should splat without errors");
        let val = stack.pop_value();
        let expected = vec_from_lanes(vec![2u64, 2u64]);
        assert_eq!(val, Some(Val::Vec(expected)), "should property splat");
    }

    #[test]
    fn shape_extract_lane_s_i8x16_test() {
        let mut stack = Stack::new();
        stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(vec![
            255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]))));

        i8x16_extract_lane_s(&mut stack, 0).expect("should splat without errors");

        let val = stack.pop_value();
        assert_eq!(val, Some(Val::I32(-1i32 as u32)), "should property splat");
    }

    #[test]
    fn shape_extract_lane_u_i8x16_test() {
        let mut stack = Stack::new();
        stack.push_entry(StackEntry::Value(Val::Vec(vec_from_lanes(vec![
            255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]))));

        i8x16_extract_lane_u(&mut stack, 0).expect("should splat without errors");

        let val = stack.pop_value();
        assert_eq!(val, Some(Val::I32(255)), "should property splat");
    }
}
