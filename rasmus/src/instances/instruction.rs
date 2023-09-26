use syntax::traits::{AsSigned, Max, Min};
use syntax::types::FuncIdx;

use super::label::LabelInst;
use super::ref_inst::RefInst;
use super::stack::StackEntry;
use super::value::Val;
use super::{frame::Frame, stack::Stack};
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
macro_rules! iextend_s {
    ($from_type: ty, $signed_type: ty) => {
        |val: $from_type| -> $from_type { (val as $signed_type) as $from_type }
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
        if let Some($first_type(second)) = $stack.pop_value() {
            if let Some($second_type(first)) = $stack.pop_value() {
                let result = ($($op)*)(first, second)?;
                $stack.push_entry(StackEntry::Value($ret(result)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    };
    ($stack: expr, $type: path, $($op: tt)*) => {
        if let Some($type(second)) = $stack.pop_value() {
            if let Some($type(first)) = $stack.pop_value() {
                let result = ($($op)*)(first, second)?;
                $stack.push_entry(StackEntry::Value($type(result)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! binop_with_value {
    ($stack: expr, $type: path, $val: expr, $($op: tt)*) => {
        if let Some($type(second)) = $stack.pop_value() {
            if let Some($type(first)) = $stack.pop_value() {
                let result = ($($op)*)(first, second, $val)?;
                $stack.push_entry(StackEntry::Value($type(result)));
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
    Ok(((a as u128) + base - (b as u128)).rem_euclid(base) as u32)
}
pub fn isub_64(a: u64, b: u64) -> RResult<u64> {
    let base = 2u128.pow(64);
    Ok(((a as u128) + base - (b as u128)).rem_euclid(base) as u64)
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
    if div == 2u32.pow(31) as i32 {
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
    let div = a_s / b_s;
    if div == 2u64.pow(63) as i64 {
        return Err(Trap);
    }
    Ok(div as u64)
}

pub fn irem_32_u(a: u32, b: u32) -> RResult<u32> {
    if b == 0 {
        return Err(Trap);
    }

    Ok(a - b * (a / b))
}

pub fn irem_32_s(a: u32, b: u32) -> RResult<u32> {
    let a_s = a as i32;
    let b_s = b as i32;
    if b_s == 0 {
        return Err(Trap);
    }

    Ok((a_s - b_s * (a_s / b_s)) as u32)
}

pub fn irem_64_u(a: u64, b: u64) -> RResult<u64> {
    if b == 0 {
        return Err(Trap);
    }

    Ok(a - b * (a / b))
}

pub fn irem_64_s(a: u64, b: u64) -> RResult<u64> {
    let a_s = a as i64;
    let b_s = b as i64;
    if b_s == 0 {
        return Err(Trap);
    }

    Ok((a_s - b_s * (a_s / b_s)) as u64)
}

pub fn iand<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitAnd<Output = T>,
{
    Ok(lhs & rhs)
}

pub fn iandnot<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: std::ops::BitAnd<Output = T> + std::ops::Not<Output = T>,
{
    Ok(lhs & !rhs)
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

pub fn bitselect<T>(first: T, second: T, third: T) -> RResult<T>
where
    T: ::std::ops::Not<Output = T>
        + ::std::ops::BitAnd<Output = T>
        + ::std::ops::BitOr<Output = T>
        + ::std::marker::Copy,
{
    Ok((first & third) | (second & !third))
}

pub fn ishl_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    let shifted = lhs << k;
    Ok((shifted as u128).rem_euclid((2u128).pow(32)) as u32)
}

pub fn ishl_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    let shifted = lhs << k;
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
    let bit = 0b1111111111111111111111111111111111111111111111111111111111111110;
    let mut res = lhs;
    for _ in 0..k {
        res = (res & bit).rotate_right(1);
    }

    Ok(res)
}

pub fn ishr_s_32(lhs: u32, rhs: u32) -> RResult<u32> {
    let k = rhs.rem_euclid(32);
    if lhs.leading_ones() > 0 {
        let most_significant_bit = 0b10000000000000000000000000000000u32;
        let mut res = lhs;
        for _ in 0..k {
            res = res.rotate_right(1) | most_significant_bit;
        }

        Ok(res)
    } else {
        ishr_u_32(lhs, rhs)
    }
}

pub fn ishr_s_64(lhs: u64, rhs: u64) -> RResult<u64> {
    let k = rhs.rem_euclid(64);
    if lhs.leading_ones() > 0 {
        let most_significant_bit =
            0b1000000000000000000000000000000000000000000000000000000000000000u64;
        let mut res = lhs;
        for _ in 0..k {
            res = res.rotate_right(1) | most_significant_bit;
        }

        Ok(res)
    } else {
        ishr_u_64(lhs, rhs)
    }
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

pub fn fadd<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Add<Output = T>,
{
    Ok(lhs + rhs)
}

pub fn fsub<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Sub<Output = T>,
{
    Ok(lhs - rhs)
}

pub fn fmul<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Mul<Output = T>,
{
    Ok(lhs * rhs)
}

pub fn fdiv<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: ::std::ops::Div<Output = T>,
{
    Ok(lhs / rhs)
}

pub fn eq<L, R>(lhs: L, rhs: R) -> RResult<u32>
where
    L: ::std::cmp::PartialEq<R>,
{
    Ok(if lhs == rhs { 1 } else { 0 })
}

pub fn neq<L, R>(lhs: L, rhs: R) -> RResult<u32>
where
    L: ::std::cmp::PartialEq<R>,
{
    Ok(if lhs != rhs { 1 } else { 0 })
}

pub fn eqz<T>(lhs: T) -> RResult<u32>
where
    T: Into<u64>,
{
    Ok(if lhs.into() == 0u64 { 1 } else { 0 })
}

pub fn lts<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) < (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn ltu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs < rhs { 1 } else { 0 })
}

pub fn gts<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) > (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn gtu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs > rhs { 1 } else { 0 })
}

pub fn min<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: Min,
{
    Ok(lhs.get_min(rhs))
}

pub fn max<T>(lhs: T, rhs: T) -> RResult<T>
where
    T: Max,
{
    Ok(lhs.get_max(rhs))
}

pub fn les<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) <= (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn leu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs <= rhs { 1 } else { 0 })
}

pub fn ges<T, O>(lhs: T, rhs: T) -> RResult<u32>
where
    T: AsSigned<Output = O>,
    O: ::std::cmp::PartialOrd,
{
    Ok(if (lhs.as_signed()) >= (rhs.as_signed()) {
        1
    } else {
        0
    })
}

pub fn geu<T>(lhs: T, rhs: T) -> RResult<u32>
where
    T: ::std::cmp::PartialOrd,
{
    Ok(if lhs >= rhs { 1 } else { 0 })
}

pub fn ref_func(stack: &mut Stack, func_idx: usize) -> RResult<()> {
    match stack.current_frame() {
        Some(frame) => match frame.module.funcaddrs.get(func_idx) {
            Some(funcaddr) => {
                stack.push_entry(StackEntry::Value(Val::Ref(RefInst::Func(funcaddr.clone()))))
            }
            None => {
                return Err(Trap);
            }
        },
        None => {
            return Err(Trap);
        }
    }

    Ok(())
}

pub fn is_ref_null(stack: &mut Stack) -> RResult<()> {
    if let Some(Val::Ref(reference)) = stack.pop_value() {
        let is_null = match reference {
            RefInst::Null(_) => 1u32,
            _ => 0u32,
        };
        stack.push_entry(StackEntry::Value(Val::I32(is_null)));
    } else {
        return Err(Trap);
    }

    Ok(())
}

#[macro_export]
macro_rules! binop_impl {
    // ($fn_name:ident, $first_type: ty, $second_type: ty, $ret: ty) => {
    //     fn $fn_name(
    //         exec_fn: impl FnOnce($first_type, $second_type) -> $ret,
    //         stack: &mut Stack,
    //     ) -> RResult<()> {
    //         if let Some($first_type(second)) = stack.pop_value() {
    //             if let Some($second_type(first)) = stack.pop_value() {
    //                 let result = exec_fn(first, second)?;
    //                 stack.push_entry(StackEntry::Value(result));
    //             } else {
    //                 return Err(Trap);
    //             }
    //         } else {
    //             return Err(Trap);
    //         }
    //     }
    // };
    ($fn_name:ident, $pattern: path, $type: ty) => {
        #[inline]
        fn $fn_name(
            exec_fn: impl FnOnce($type, $type) -> RResult<$type>,
            stack: &mut Stack,
        ) -> RResult<()> {
            if let Some($pattern(second)) = stack.pop_value() {
                if let Some($pattern(first)) = stack.pop_value() {
                    let result = exec_fn(first, second)?;
                    stack.push_entry(StackEntry::Value($pattern(result)));
                    return Ok(());
                } else {
                    return Err(Trap);
                }
            } else {
                return Err(Trap);
            }
        }
    };
}

#[macro_export]
macro_rules! testop_impl {
    ($fn_name:ident, $pattern: path, $type: ty) => {
        #[inline]
        fn $fn_name(exec_fn: impl FnOnce($type) -> RResult<u32>, stack: &mut Stack) -> RResult<()> {
            if let Some($pattern(first)) = stack.pop_value() {
                let result = exec_fn(first)?;
                stack.push_entry(StackEntry::Value(crate::instances::value::Val::I32(result)));
            } else {
                return Err(Trap);
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! relop_impl {
    ($fn_name:ident, $pattern: path, $type: ty) => {
        #[inline]
        fn $fn_name(
            exec_fn: impl FnOnce($type, $type) -> RResult<u32>,
            stack: &mut Stack,
        ) -> RResult<()> {
            if let Some($pattern(second)) = stack.pop_value() {
                if let Some($pattern(first)) = stack.pop_value() {
                    let result = exec_fn(first, second)?;
                    stack.push_entry(StackEntry::Value(crate::instances::value::Val::I32(result)));
                } else {
                    return Err(Trap);
                }
            } else {
                return Err(Trap);
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! relop {
    ($stack: expr, $arg_type: path, $($op: tt)*) => {
        if let Some($arg_type(second)) = $stack.pop_value() {
            if let Some($arg_type(first)) = $stack.pop_value() {
                let result = ($($op)*)(first, second)?;
                $stack.push_entry(StackEntry::Value(crate::instances::value::Val::I32(result)));
            } else {
                return Err(Trap);
            }
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! cvtop {
    ($stack: expr, $arg_type: path, $ret_type: path, $($op: tt)*) => {
        if let Some($arg_type(arg)) = $stack.pop_value() {
            let result = ($($op)*)(arg)?;
            $stack.push_entry(StackEntry::Value($ret_type(result)));
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! trunc_u {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg == <$arg_type>::NAN || arg == <$arg_type>::INFINITY {
                return Err(Trap);
            }

            let trunked = arg.trunc() as u128;
            <$ret_type>::try_from(trunked).map_err(|_| Trap)
        }
    };
}

#[macro_export]
macro_rules! trunc_s {
    ($arg_type: ty, $aux_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg == <$arg_type>::NAN || arg == <$arg_type>::INFINITY {
                return Err(Trap);
            }

            let trunked = arg.trunc() as u128;
            <$aux_type>::try_from(trunked)
                .map_err(|_| Trap)
                .map(|v| v as $ret_type)
        }
    };
}

#[macro_export]
macro_rules! trunc_sat_u {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg == <$arg_type>::NAN {
                return Err(Trap);
            }

            if arg == <$arg_type>::NEG_INFINITY {
                return Ok(0);
            }

            if arg == <$arg_type>::INFINITY {
                return Ok(<$ret_type>::MAX);
            }

            <$ret_type>::try_from(arg.trunc() as u128).or_else(|_| Ok(<$ret_type>::MAX))
        }
    };
}

#[macro_export]
macro_rules! fmin {
    ($type: ty) => {
        |lhs: $type, rhs: $type| {
            if lhs.is_nan() || rhs.is_nan() {
                return Ok(<$type>::NAN);
            }
            Ok(lhs.min(rhs))
        }
    };
}

#[macro_export]
macro_rules! fmax {
    ($type: ty) => {
        |lhs: $type, rhs: $type| {
            if lhs.is_nan() || rhs.is_nan() {
                return Ok(<$type>::NAN);
            }
            Ok(lhs.max(rhs))
        }
    };
}

#[macro_export]
macro_rules! fcopysign {
    ($type: ty) => {
        |lhs: $type, rhs: $type| Ok(lhs.copysign(rhs))
    };
}

#[macro_export]
macro_rules! trunc_sat_s {
    ($arg_type: ty, $aux_type: ty, $ret_type: ty) => {
        |arg: $arg_type| {
            if arg == <$arg_type>::NAN {
                return Err(Trap);
            }

            if arg == <$arg_type>::NEG_INFINITY {
                return Ok(<$aux_type>::MIN as $ret_type);
            }

            if arg == <$arg_type>::INFINITY {
                return Ok(<$aux_type>::MAX as $ret_type);
            }

            let trunced = arg.trunc() as i128;

            if trunced > (<$aux_type>::MAX as i128) {
                return Ok(<$aux_type>::MAX as $ret_type);
            }

            if trunced < (<$aux_type>::MIN as i128) {
                return Ok(<$aux_type>::MIN as $ret_type);
            }

            Ok(trunced as $ret_type)
        }
    };
}

// Rust float is already defined in IEEE 754 standard, so using `as`.
#[macro_export]
macro_rules! float {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| arg as $ret_type
    };
}

#[macro_export]
macro_rules! float_u {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| Ok($crate::float!($arg_type, $ret_type)(arg))
    };
}

#[macro_export]
macro_rules! float_s {
    ($arg_type: ty, $aux_type: ty, $ret_type: ty) => {
        |arg: $arg_type| Ok($crate::float!($aux_type, $ret_type)(arg as $aux_type))
    };
}

#[macro_export]
macro_rules! demote {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| Ok(arg as $ret_type)
    };
}

#[macro_export]
macro_rules! promote {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| Ok(arg as $ret_type)
    };
}

#[macro_export]
macro_rules! reinterpret {
    ($arg_type: ty, $ret_type: ty) => {
        |arg: $arg_type| -> RResult<$ret_type> {
            let mut bytes = arg.to_le_bytes();
            Ok(::syntax::read_unsigned_leb128!($ret_type)(
                &mut bytes,
                &mut 0usize,
            ))
        }
    };
}

#[macro_export]
macro_rules! is_ref_null {
    ($stack: expr) => {
        if let Some($crate::instances::value::Val::Ref(reference)) = $stack.pop_value() {
            let is_null = match reference {
                $crate::instances::ref_inst::RefInst::Null(_) => 1u32,
                _ => 0u32,
            };
            $stack.push_entry(StackEntry::Value(Val::I32(is_null)));
        } else {
            return Err(Trap);
        }
    };
}

#[macro_export]
macro_rules! ref_func_m {
    ($stack: expr, $func_idx: expr) => {
        match $stack.current_frame() {
            Some(frame) => match frame.module.funcaddrs.get($func_idx) {
                Some(funcaddr) => $stack.push_entry(StackEntry::Value(Val::Ref(
                    $crate::instances::ref_inst::RefInst::Func(funcaddr.clone()),
                ))),
                None => {
                    return Err(Trap);
                }
            },
            None => {
                return Err(Trap);
            }
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test_float_f32_from_u32() {
        let make_float = float!(u32, f32);

        assert_eq!(make_float(0), 0.0f32, "should properly convert zero");
        assert_eq!(
            make_float(1),
            1.0f32,
            "should properly convert exact number"
        );
        assert_eq!(
            make_float(1),
            1.0f32,
            "should properly convert exact number"
        );
    }
}
