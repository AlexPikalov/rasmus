#[macro_export]
macro_rules! extract_lane_stack_type {
    (i8 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i32()],
        }
    };
    (i16 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i32()],
        }
    };
    (i32 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i32()],
        }
    };
    (i64 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::i64()],
        }
    };
    (f32 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::f32()],
        }
    };
    (f64 $dim:expr, $input:expr) => {
        StackType {
            inputs: $input,
            outputs: vec![ValType::f64()],
        }
    };
}

#[macro_export]
macro_rules! replace_lane_stack_type {
    (i8 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (i16 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (i32 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (i64 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::i64()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (f32 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::f32()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
    (f64 $dim:expr, $input:expr) => {
        StackType {
            inputs: vec![
                OpdType::Strict(ValType::v128()),
                OpdType::Strict(ValType::f64()),
            ],
            outputs: vec![ValType::v128()],
        }
    };
}

#[macro_export]
macro_rules! check {
  (extract_lane $n:ident, $dim:expr, $lane_idx:expr) => {{
      let dim: u8 = $dim;

      if $lane_idx.0 >= dim {
          return Err(ValidationError::LaneIndexIsOutOfRange {
              value: $lane_idx.0,
              max_allowed: dim - 1,
          });
      }

      crate::extract_lane_stack_type!($n $dim,  vec![OpdType::Strict(ValType::v128())])
  }};
  (replace_lane $n:ident, $dim:expr, $lane_idx:expr) => {{
      let dim: u8 = $dim;

      if $lane_idx.0 >= dim {
          return Err(ValidationError::LaneIndexIsOutOfRange {
              value: $lane_idx.0,
              max_allowed: dim - 1,
          });
      }

      crate::replace_lane_stack_type!($n $dim,  vec![OpdType::Strict(ValType::v128())])
  }};
}
