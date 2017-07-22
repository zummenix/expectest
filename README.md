# expectest
[![Build Status](https://travis-ci.org/zummenix/expectest.svg?branch=master)](https://travis-ci.org/zummenix/expectest)
[![Crates.io](https://img.shields.io/crates/v/expectest.svg)](https://crates.io/crates/expectest)
[![Crates.io](https://img.shields.io/crates/d/expectest.svg)](https://crates.io/crates/expectest)

Crate provides matchers and matcher functions for unit testing. Inspired by [Nimble](https://github.com/Quick/Nimble) for Swift.

Using this library you will receive helpful messages with data that is used in test case, example:
```rust,should_panic
let result = vec![1, 2, 2];
expect!(result).to(be_equal_to([1, 2, 3]));
```
Test fails and gives us a message:
```
expected to be equal to <[1, 2, 3]>, got <[1, 2, 2]>
```

### Examples

Check out the [tests](/tests) directory!

### Changes

Take a look at [change log](CHANGELOG.md).

### Alternative crates
- [spectral](https://github.com/cfrancia/spectral)
- [rustspec-assertions](https://github.com/uorbe001/rustspec-assertions)
- [hamcrest-rust](https://github.com/carllerche/hamcrest-rust)


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
