# expectest
[![](http://meritbadge.herokuapp.com/expectest)](https://crates.io/crates/expectest)

Library provides functions for unit testing with human-readable syntax. Inspired by [Nimble](https://github.com/Quick/Nimble) for Swift.

*Work In Progress*

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
