use super::frame::Frame;
use super::label::LabelInst;
use crate::{
    address::{ExternAddr, FuncAddr},
    result::{RResult, Trap},
};

#[derive(Debug)]
pub enum InstructionInst {
    Trap,
    Ref(FuncAddr),
    RefExtern(ExternAddr),
    Invoke(FuncAddr),
    Label(LabelInst),
    Frame(Frame),
    End,
}

#[macro_export]
macro_rules! iextend {
    ($itype: ty, $base: expr) => {
        |v: $itype| {
            let mut val_bytes = v.to_le_bytes();
            let base_num_bytes = $base / 8;
            let trailing_byte = if val_bytes[base_num_bytes - 1].leading_ones() > 0 {
                255u8
            } else {
                0u8
            };
            for byte in 0..v.to_le_bytes().len() {
                if byte >= base_num_bytes {
                    val_bytes[byte] = trailing_byte
                }
            }
            <$itype>::from_le_bytes(val_bytes)
        }
    };
}

#[macro_export]
macro_rules! nearest {
    ($ftype:ty) => {
        |v: $ftype| {
            if v == <$ftype>::NAN || v == <$ftype>::INFINITY || v == 0.0 {
                v
            } else if v > 0.0 && v <= 0.5 {
                -0.0
            } else if v < 0.0 && v >= -0.5 {
                0.0
            } else if v < 0.0 {
                if v - v.trunc() >= -0.5 {
                    v.trunc()
                } else {
                    v.trunc() + 1.0
                }
            } else {
                if v - v.trunc() <= 0.5 {
                    v.trunc()
                } else {
                    v.trunc() + 1.0
                }
            }
        }
    };
}

#[macro_export]
macro_rules! binop {
    ($stack: expr, $first_type: path, $second_type: path, $ret: path, $($op: tt)*) => {
        if let Some($first_type(first)) = $stack.pop_value() {
            if let Some($second_type(second)) = $stack.pop_value() {
                let result = ($($op)*)(first, second)?;
                $stack.push_entry(StackEntry::Value($ret(result)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    };
}

pub fn iadd_32(a: u32, b: u32) -> RResult<u32> {
    Ok(((a as u128) + (b as u128)).rem_euclid(2u128.pow(32)) as u32)
}
pub fn iadd_64(a: u64, b: u64) -> RResult<u64> {
    Ok(((a as u128) + (b as u128)).rem_euclid(2u128.pow(64)) as u64)
}

pub fn isub_32(a: u32, b: u32) -> RResult<u32> {
    let base = 2u128.pow(32);
    Ok(((a as u128) + (b as u128) + base).rem_euclid(base) as u32)
}
pub fn isub_64(a: u64, b: u64) -> RResult<u64> {
    let base = 2u128.pow(64);
    Ok(((a as u128) - (b as u128) + base).rem_euclid(base) as u64)
}

pub fn imul_32(a: u32, b: u32) -> RResult<u32> {
    let base = 2u128.pow(32);
    Ok(((a as u128) * (b as u128)).rem_euclid(base) as u32)
}
pub fn imul_64(a: u64, b: u64) -> RResult<u64> {
    let base = 2u128.pow(64);
    Ok(((a as u128) * (b as u128)).rem_euclid(base) as u64)
}

pub fn idiv_32_u(a: u32, b: u32) -> RResult<u32> {
    if b == 0 {
        return Err(Trap);
    }
    Ok(a.div_euclid(b))
}

pub fn idiv_32_s(a: u32, b: u32) -> RResult<u32> {
    let a_s = a as i32;
    let b_s = b as i32;
    if b_s == 0 {
        return Err(Trap);
    }
    let div = a_s.div_euclid(b_s);
    if div == 2i32.pow(31) {
        return Err(Trap);
    }
    Ok(div as u32)
}

pub fn idiv_64_u(a: u64, b: u64) -> RResult<u64> {
    if b == 0 {
        return Err(Trap);
    }
    Ok(a.div_euclid(b))
}

pub fn idiv_64_s(a: u64, b: u64) -> RResult<u64> {
    let a_s = a as i64;
    let b_s = b as i64;
    if b_s == 0 {
        return Err(Trap);
    }
    let div = a_s.div_euclid(b_s);
    if div == 2i64.pow(63) {
        return Err(Trap);
    }
    Ok(div as u64)
}

pub fn irem_32_u(a: u32, b: u32) -> RResult<u32> {
    if b == 0 {
        return Err(Trap);
    }

    Ok(a.rem_euclid(b))
}

pub fn irem_32_s(a: u32, b: u32) -> RResult<u32> {
    let a_s = a as i32;
    let b_s = b as i32;
    if b_s == 0 {
        return Err(Trap);
    }

    Ok(a_s.rem_euclid(b_s) as u32)
}

pub fn irem_64_u(a: u64, b: u64) -> RResult<u64> {
    if b == 0 {
        return Err(Trap);
    }

    Ok(a.rem_euclid(b))
}

pub fn irem_64_s(a: u64, b: u64) -> RResult<u64> {
    let a_s = a as i64;
    let b_s = b as i64;
    if b_s == 0 {
        return Err(Trap);
    }

    Ok(a_s.rem_euclid(b_s) as u64)
}

pub fn iand<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitAnd<Output = T>,
{
    Ok(lhs & rhs)
}

pub fn ior<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitOr<Output = T>,
{
    Ok(lhs | rhs)
}

pub fn ixor<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitXor<Output = T>,
{
    Ok(lhs ^ rhs)
}

pub fn ishl_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    let shifted = lhs >> k;
    Ok((shifted as u128).rem_euclid((2u128).pow(32)) as u32)
}

pub fn ishl_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    let shifted = lhs >> k;
    Ok((shifted as u128).rem_euclid((2u128).pow(64)) as u64)
}

pub fn ishr_u_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    let bit = 0b11111111111111111111111111111110;
    let mut res = lhs;
    for _ in 0..k {
        res = (res & bit).rotate_right(1);
    }

    Ok(res)
}

pub fn ishr_u_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    let bit = 0b11111111111111111111111111111110;
    let mut res = lhs;
    for _ in 0..k {
        res = (res & bit).rotate_right(1);
    }

    Ok(res)
}

pub fn ishr_s_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    let most_significant_bit = if lhs.leading_ones() > 0 {
        0b11111111111111111111111111111111
    } else {
        0b11111111111111111111111111111110
    };
    let mut res = lhs;
    for _ in 0..k {
        res = (res | most_significant_bit).rotate_right(1);
    }

    Ok(res)
}

pub fn ishr_s_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    let most_significant_bit = if lhs.leading_ones() > 0 {
        0b11111111111111111111111111111111
    } else {
        0b11111111111111111111111111111110
    };
    let mut res = lhs;
    for _ in 0..k {
        res = (res | most_significant_bit).rotate_right(1);
    }

    Ok(res)
}

pub fn irotl_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    Ok(lhs.rotate_left(k))
}

pub fn irotl_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64) as u32;
    Ok(lhs.rotate_left(k))
}

pub fn irotr_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    Ok(lhs.rotate_right(k))
}

pub fn irotr_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64) as u32;
    Ok(lhs.rotate_right(k))
}

#[macro_export]
macro_rules! testop {
    ($stack: expr, $first_type: path, $ret: path, $($op: tt)*) => {
        if let Some($first_type(first)) = $stack.pop_value() {
            let result = ($($op)*)(first)?;
            $stack.push_entry(StackEntry::Value($ret(result)));
        } else {
            return Err(Trap);
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test_iextend() {
        assert_eq!(
            iextend!(i32, 8)(0b11110000).to_be_bytes(),
            [255u8, 255u8, 255u8, 0b11110000u8]
        );
        assert_eq!(
            iextend!(i32, 8)(0b01110000).to_be_bytes(),
            [0u8, 0u8, 0u8, 0b01110000u8]
        );
    }
}
