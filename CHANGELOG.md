# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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
