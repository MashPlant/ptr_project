This crate is inspired by [pin-project](https://github.com/taiki-e/pin-project). It enables safe and ergonomic pointer projection.

## Usage

Currently you need to enable feature `arbitrary_self_types` to use this proc macro.

Example:

```rust
#![feature(arbitrary_self_types)]

#[ptr_project::ptr_project]
struct Struct<T, U> {
  t: T,
  u: U,
}

impl<T, U> Struct<T, U> {
  fn method(self: *const Self) {
    let this = self.project();
    let _: *const T = this.t;
    let _: *const U = this.u;
  }
}
```