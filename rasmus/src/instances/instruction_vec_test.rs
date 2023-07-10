use syntax::types::LaneIdx;

use super::instruction_vec::*;

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
fn swizzle_i8x16_test() {
    let src = vec_from_lanes(vec![
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    let pick = vec_from_lanes(vec![0u8, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15]);
    let expected = vec_from_lanes(vec![
        1u8, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16,
    ]);
    assert_eq!(
        swizzle_i8x16(src, pick).expect("should swizzle without errors"),
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
        shuffle_i8x16(
            left,
            right,
            &([33u8; 16].iter().map(|v| LaneIdx(*v)).collect())
        )
        .is_err(),
        "should return error if any of lane_idx is more or equal 32"
    );

    assert_eq!(
        shuffle_i8x16(
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
fn shape_splat_integer_test() {
  
}
