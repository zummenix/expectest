# expectest
[![Build Status](https://travis-ci.org/zummenix/expectest.svg?branch=master)]
(https://travis-ci.org/zummenix/expectest)
[![Crates.io](https://img.shields.io/crates/v/expectest.svg)](https://crates.io/crates/expectest)
[![Crates.io](https://img.shields.io/crates/d/expectest.svg)](https://crates.io/crates/expectest)

Crate provides matchers and matcher functions for unit testing. Inspired by [Nimble](https://github.com/Quick/Nimble) for Swift.

Using this library you will receive helpful messages with data that is used in test case, example:
```rust
let result = vec![1, 2, 2];
expect!(result).to(be_equal_to([1, 2, 3]));
```
Test fails and gives us a message:
```
expected to be equal to <[1, 2, 3]>, got <[1, 2, 2]>
```

### Usage

In Cargo.toml:
```toml
[dev-dependencies]
expectest = "~0.2.3"
```

In your crate:
```rust
#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;
```

You can export all needed functions and types from `prelude` module:
```rust
use expectest::prelude::*;
```

### Examples

Check out the [tests](/tests) directory!


### Expectations
Use basic syntax to express expectations:
```rust
expect!(...).to(...);
expect!(...).to_not(...);
expect!(...).not_to(...);
```
> Note: You can use `expect` function instead of `expect!` macro in that case you will not
see file and line where the test will fail.

#### Equality
For types that implement the `PartialEq` trait:
```rust
expect!("hello".to_string()).to(be_equal_to("hello"));
```

#### Closeness of float numbers
There is a way to check if two float numbers are close each other:
```rust
expect!(12.1_f64).to(be_close_to(12.0).delta(0.1));
```
With default `delta` equal `0.001`:
```rust
expect!(12.001_f64).to(be_close_to(12.0));
```

#### Order
For types that implement the `PartialOrd` trait:
```rust
expect!(1).to(be_greater_than(0));
```
Use any of the following matchers: `be_less_than`, `be_less_or_equal_to`, `be_greater_than`, `be_greater_or_equal_to`

#### Option
There are matchers for the `Option<T>` type:
```rust
expect!(Some(9)).to(be_some().value(9));
```
Use any of the following matchers: `be_some`, `be_none`

#### Result
There are matchers for the `Result<T, E>` type:
```rust
expect!("4".parse::<u32>()).to(be_ok().value(4));
```
Use any of the following matchers: `be_ok`, `be_err`

#### Emptyness
There is `be_empty` matcher for types that implement `IsEmpty` trait:
```rust
expect!("").to(be_empty());
```
> Note: `IsEmpty` trait implemented by library for following types:
`String`, `&str`, `Vec<T>`, `&[T]`.

#### Boolean
```rust
expect!(9 == 9).to(be_true());
```
Use any of the following matchers: `be_true`, `be_false`

### Alternative crates
- [rustspec-assertions](https://github.com/uorbe001/rustspec-assertions)
- [hamcrest-rust](https://github.com/carllerche/hamcrest-rust)

## License
MIT
