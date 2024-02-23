(component
  (type (;0;)
    (instance
      (type (;0;) (record (field "a" u8)))
      (export (;1;) "x" (type (eq 0)))
      (type (;2;) (func (param "x" string)))
      (export (;0;) "bar1" (func (type 2)))
      (type (;3;) (func (param "x" 1)))
      (export (;1;) "bar2" (func (type 3)))
    )
  )
  (import "bar" (instance (;0;) (type 0)))
  (type (;1;)
    (instance
      (type (;0;) s8)
      (export (;1;) "x" (type (eq 0)))
      (type (;2;) (list string))
      (type (;3;) (func (param "x" 2)))
      (export (;0;) "baz1" (func (type 3)))
      (type (;4;) (func))
      (export (;1;) "baz2" (func (type 4)))
      (type (;5;) (func (param "x" 1)))
      (export (;2;) "baz3" (func (type 5)))
    )
  )
  (import "baz" (instance (;1;) (type 1)))
  (type (;2;)
    (instance
      (type (;0;) (func))
      (export (;0;) "foo1" (func (type 0)))
      (type (;1;) (func (param "x" u8)))
      (export (;1;) "foo2" (func (type 1)))
      (type (;2;) (func (param "x" float32)))
      (export (;2;) "foo3" (func (type 2)))
    )
  )
  (import "foo" (instance (;2;) (type 2)))
  (core module (;0;)
    (type (;0;) (func))
    (type (;1;) (func (param i32)))
    (type (;2;) (func (param f32)))
    (type (;3;) (func (param i32 i32)))
    (type (;4;) (func (param i32 i32 i32 i32) (result i32)))
    (import "foo" "foo1" (func (;0;) (type 0)))
    (import "foo" "foo2" (func (;1;) (type 1)))
    (import "foo" "foo3" (func (;2;) (type 2)))
    (import "bar" "bar1" (func (;3;) (type 3)))
    (import "bar" "bar2" (func (;4;) (type 1)))
    (import "baz" "baz1" (func (;5;) (type 3)))
    (import "baz" "baz2" (func (;6;) (type 0)))
    (import "baz" "baz3" (func (;7;) (type 1)))
    (func (;8;) (type 4) (param i32 i32 i32 i32) (result i32)
      unreachable
    )
    (memory (;0;) 1)
    (export "memory" (memory 0))
    (export "cabi_realloc" (func 8))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
      (processed-by "my-fake-bindgen" "123.45")
    )
  )
  (core module (;1;)
    (type (;0;) (func (param i32 i32)))
    (func $indirect-bar-bar1 (;0;) (type 0) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 0
      call_indirect (type 0)
    )
    (func $indirect-baz-baz1 (;1;) (type 0) (param i32 i32)
      local.get 0
      local.get 1
      i32.const 1
      call_indirect (type 0)
    )
    (table (;0;) 2 2 funcref)
    (export "0" (func $indirect-bar-bar1))
    (export "1" (func $indirect-baz-baz1))
    (export "$imports" (table 0))
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core module (;2;)
    (type (;0;) (func (param i32 i32)))
    (import "" "0" (func (;0;) (type 0)))
    (import "" "1" (func (;1;) (type 0)))
    (import "" "$imports" (table (;0;) 2 2 funcref))
    (elem (;0;) (i32.const 0) func 0 1)
    (@producers
      (processed-by "wit-component" "$CARGO_PKG_VERSION")
    )
  )
  (core instance (;0;) (instantiate 1))
  (alias export 2 "foo1" (func (;0;)))
  (core func (;0;) (canon lower (func 0)))
  (alias export 2 "foo2" (func (;1;)))
  (core func (;1;) (canon lower (func 1)))
  (alias export 2 "foo3" (func (;2;)))
  (core func (;2;) (canon lower (func 2)))
  (core instance (;1;)
    (export "foo1" (func 0))
    (export "foo2" (func 1))
    (export "foo3" (func 2))
  )
  (alias core export 0 "0" (core func (;3;)))
  (alias export 0 "bar2" (func (;3;)))
  (core func (;4;) (canon lower (func 3)))
  (core instance (;2;)
    (export "bar1" (func 3))
    (export "bar2" (func 4))
  )
  (alias core export 0 "1" (core func (;5;)))
  (alias export 1 "baz2" (func (;4;)))
  (core func (;6;) (canon lower (func 4)))
  (alias export 1 "baz3" (func (;5;)))
  (core func (;7;) (canon lower (func 5)))
  (core instance (;3;)
    (export "baz1" (func 5))
    (export "baz2" (func 6))
    (export "baz3" (func 7))
  )
  (core instance (;4;) (instantiate 0
      (with "foo" (instance 1))
      (with "bar" (instance 2))
      (with "baz" (instance 3))
    )
  )
  (alias core export 4 "memory" (core memory (;0;)))
  (alias core export 4 "cabi_realloc" (core func (;8;)))
  (alias core export 0 "$imports" (core table (;0;)))
  (alias export 0 "bar1" (func (;6;)))
  (core func (;9;) (canon lower (func 6) (memory 0) string-encoding=utf8))
  (alias export 1 "baz1" (func (;7;)))
  (core func (;10;) (canon lower (func 7) (memory 0) string-encoding=utf8))
  (core instance (;5;)
    (export "$imports" (table 0))
    (export "0" (func 9))
    (export "1" (func 10))
  )
  (core instance (;6;) (instantiate 2
      (with "" (instance 5))
    )
  )
  (@producers
    (processed-by "wit-component" "$CARGO_PKG_VERSION")
  )
)
