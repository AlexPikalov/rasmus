use syntax::traits::AsSigned;

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

pub fn bitselect<T>(first: T, second: T, third: T) -> RResult<T>
where
    T: ::std::ops::Not<Output = T>
        + ::std::ops::BitAnd<Output = T>
        + ::std::ops::BitOr<Output = T>
        + ::std::marker::Copy,
{
    Ok((first & third) | (second & !third))
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
macro_rules! cvtop_impl {
    ($fn_name:ident, $input_val: path, $input_type: ty, $output_val: path, $output_type: ty) => {
        #[inline]
        fn $fn_name(
            exec_fn: impl FnOnce($input_type) -> RResult<$output_type>,
            stack: &mut Stack,
        ) -> RResult<()> {
            if let Some($input_val(arg)) = stack.pop_value() {
                let result = exec_fn(arg)?;
                stack.push_entry(StackEntry::Value($output_val(result)));
            } else {
                return Err(Trap);
            }
            Ok(())
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
