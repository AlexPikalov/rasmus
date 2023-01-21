(module
  ;; imports
  (import "foo" "bar" (func (param f32)))
  ;; globals
  (global (import "js" "global") (mut i32))
  ;; funcs
  (func)
  (func ()
    i32.const 42
    drop)
  ;; tables
  (table 0 1 funcref)
  ;; mems
  (memory (data "hi"))
  
  ;; exports
  (export "e" (func 1))
  ;; start
  (start 1)
)