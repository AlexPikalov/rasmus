use super::{
    instructions::{ExpressionType, InstructionType},
    parse_trait::ParseWithNom,
    types::*,
};

use nom::{
    bytes::complete::{take, take_till},
    IResult as NomResult, Slice,
};

pub use super::parse_instruction_fn::parse_instruction;

// Copied from https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_serialize/leb128.rs.html
macro_rules! impl_read_unsigned_leb128 {
    ($fn_name:ident, $int_ty:ty) => {
        #[inline]
        pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
            // The first iteration of this loop is unpeeled. This is a
            // performance win because this code is hot and integer values less
            // than 128 are very common, typically occurring 50-80% or more of
            // the time, even for u64 and u128.
            let byte = slice[*position];
            *position += 1;
            if (byte & 0x80) == 0 {
                return byte as $int_ty;
            }
            let mut result = (byte & 0x7F) as $int_ty;
            let mut shift = 7;
            loop {
                let byte = slice[*position];
                *position += 1;
                if (byte & 0x80) == 0 {
                    result |= (byte as $int_ty) << shift;
                    return result;
                } else {
                    result |= ((byte & 0x7F) as $int_ty) << shift;
                }
                shift += 7;
            }
        }
    };
}

impl_read_unsigned_leb128!(read_u32_leb128, u32);
impl_read_unsigned_leb128!(read_u64_leb128, u64);

macro_rules! impl_read_signed_leb128 {
    ($fn_name:ident, $int_ty:ty) => {
        #[inline]
        pub fn $fn_name(slice: &[u8], position: &mut usize) -> $int_ty {
            let mut result = 0;
            let mut shift = 0;
            let mut byte;

            loop {
                byte = slice[*position];
                *position += 1;
                result |= <$int_ty>::from(byte & 0x7F) << shift;
                shift += 7;

                if (byte & 0x80) == 0 {
                    break;
                }
            }

            if (shift < <$int_ty>::BITS) && ((byte & 0x40) != 0) {
                // sign extend
                result |= (!0 << shift);
            }

            result
        }
    };
}

impl_read_signed_leb128!(read_i16_leb128, i16);
impl_read_signed_leb128!(read_i32_leb128, i32);
impl_read_signed_leb128!(read_i64_leb128, i64);
impl_read_signed_leb128!(read_i128_leb128, i128);
impl_read_signed_leb128!(read_isize_leb128, isize);

// impl_read_signed_leb128 with 33 bytes and i64 as container type
pub fn read_s33_leb128(slice: &[u8], position: &mut usize) -> i64 {
    let mut result = 0;
    let mut shift = 0;
    let mut byte;

    loop {
        byte = slice[*position];
        *position += 1;
        result |= <i64>::from(byte & 0x7F) << shift;
        shift += 7;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    if (shift < 33) && ((byte & 0x40) != 0) {
        // sign extend
        result |= (!0 << shift);
    }

    result
}

// Unlike to Vec::parse this function should be used for cases when a number
// of structures is not-known
pub fn parse_all_to_vec<T>(bytes: &[Byte], till: Byte) -> NomResult<&[Byte], Vec<T>>
where
    T: ParseWithNom + Sized,
{
    let (bytes, bytes_to_parse) = take_till(|b| b == till)(bytes)?;
    let mut remaining_bytes = bytes_to_parse;
    let mut accumulator: Vec<T> = Vec::new();

    while !remaining_bytes.is_empty() {
        let parsed = T::parse(remaining_bytes)?;
        remaining_bytes = parsed.0;
        accumulator.push(parsed.1);
    }

    Ok((bytes.slice(1..), accumulator))
}

// TODO: create unit test for parse_all_to_vec

pub fn parse<T>(bytes: &[Byte]) -> NomResult<&[Byte], T>
where
    T: ParseWithNom,
{
    T::parse(bytes)
}
