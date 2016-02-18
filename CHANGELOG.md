# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
- Examples in the readme are tested using [skeptic](https://crates.io/crates/skeptic)

## [0.4.1] - 2016-01-12
- Minor fixes.
- Added a change log.
- Relicense to dual MIT/Apache-2.0

## [0.4.0] - 2015-11-22
- Removed reexport of types from prelude module. If you need some of those types
you need to `use` it.
- Relaxed contraints for `be_some`, `be_err`, `be_ok` matchers. Now this works:
```rust
#[derive(Debug)]
struct Foo;
expect!(Some(Foo)).to(be_some());
```
