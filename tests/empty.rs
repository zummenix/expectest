
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn emptiness_of_string() {
    expect!("".to_string()).to(be_empty());
}

#[test]
fn emptiness_of_str() {
    expect!("").to(be_empty());
}

#[test]
#[ignore]
#[should_panic]
fn emptiness_of_string_should_panic() {
    expect!("s".to_string()).to(be_empty());
}

#[test]
#[should_panic]
fn emptiness_of_str_should_panic() {
    expect!("world").to(be_empty());
}
