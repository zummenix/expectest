# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.10.0] - 2018-05-21
- Run rustfmt on all files
- Added short aliases for some long functions:
```
be_equal_to <=> be_eq
be_less_than <=> be_lt
be_less_or_equal_to <=> be_le
be_greater_than <=> be_gt
be_greater_or_equal_to <=> be_ge
```
- Set minimum rust version in `.travis.yml`

## [0.9.2] - 2018-01-21
- Adapted float point tests to a new Debug implementation

## [0.9.1] - 2017-08-18
- Changed printing to closely match rust's unit testing behavior, for example on nightly (with `nightly`
feature enabled):
```
---- core::result::tests::fails stdout ----
	thread 'tests::fails' panicked at 'assertion failed: `expected to be equal to <1>, got <0>``', src/test.rs:10:12
```
on stable:
```
---- core::result::tests::fails stdout ----
	thread 'tests::fails' panicked at 'assertion failed: `expected to be equal to <1>, got <0>``, src/test.rs:10:12', src/core/result.rs:82:12
```

## [0.8.0] - 2017-07-26
- Rework readme and docs
- Remove `skeptic` dependency

## [0.7.0] - 2017-02-11
- Update dependecies
- Add `development-tools::testing` category for crates.io
- Add travis-ci badge for crates.io

## [0.6.0] - 2016-06-11
- Implement panic using 'panic_fmt' (only for nightly rust).

## [0.5.1] - 2016-04-24
- Almost all examples in the readme are tested using [skeptic](https://crates.io/crates/skeptic)
- The `num` crate was replaced by `num-traits`
- The `be_empty` matcher now uses the `Iterator` trait analogically to the `have_count` matcher

## [0.4.1] - 2016-01-12
- Minor fixes.
- Added a change log.
- Relicense to dual MIT/Apache-2.0

## [0.4.0] - 2015-11-22
- Removed reexport of types from prelude module. If you need some of those types
you need to `use` it.
- Relaxed constraints for `be_some`, `be_err`, `be_ok` matchers. Now this works:
```rust
#[derive(Debug)]
struct Foo;
expect!(Some(Foo)).to(be_some());
```
