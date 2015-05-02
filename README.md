# expectest
[![](http://meritbadge.herokuapp.com/expectest)](https://crates.io/crates/expectest)

*Work In Progress*

Library provides functions for unit testing with human-readable syntax. Inspired by [Nimble](https://github.com/Quick/Nimble) for Swift.

One of main purposes of this library is to show what data was used in a broken test. Consider this example:
```rust
let result = ...;
expect!(result).to(be_equal_to([1, 2, 3]));
```
Assume that 'result' contains vec: `[1, 2, 2]`. Of course test fails and gives us a message:
```bash
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

Simple unit test:
```rust
expect!(Some(6)).to(be_some());
```

## License
MIT
