use super::{
    instructions::{ExpressionType, InstructionType},
    types::*,
};

pub use super::parse_instruction_fn::parse_instruction;

// Copied from https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_serialize/lib.rs.html#1-30

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
