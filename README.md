# expectest
[![](http://meritbadge.herokuapp.com/expectest)](https://crates.io/crates/expectest)

*Work In Progress*

Library provides functions for unit testing with human-readable syntax. Inspired by [Nimble](https://github.com/Quick/Nimble) for Swift.

Using this library you will receive nice messages with data that used in test case, example:
```rust
let result = vec![1, 2, 2];
expect!(result).to(be_equal_to([1, 2, 3]));
```
Test fails and gives us a message:
```
expected to be equal to <[1, 2, 3]>, got <[1, 2, 2]>
```
> Note: You need to use `cargo test -- --nocapture` to see output from tests.

### Usage

In Cargo.toml:
```toml
[dev-dependencies]
expectest = "*"
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

#### Expectations
Use basic syntax to express expectations:
```rust
expect!(...).to(...);
expect!(...).to_not(...);
expect!(...).not_to(...);
```
> Note: You can use `expect` function instead of `expect!` macro in that case you will not
see file and line where the test fails.

#### Equality
For types that implement the `PartialEq` trait:
```rust
expect!("hello".to_string()).to(be_equal_to("hello"));
```

#### Order
For types that implement the `PartialOrd` trait:
```rust
expect!(1).to(be_greater_than(0));
```
Use following matchers: `be_less_than`, `be_less_or_equal_to`, `be_greater_than`, `be_greater_or_equal_to`

### Alternative crates
- [rustspec-assertions](https://github.com/uorbe001/rustspec-assertions)
- [hamcrest-rust](https://github.com/carllerche/hamcrest-rust)

## License
MIT
