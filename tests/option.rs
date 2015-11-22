
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn to_be_some() {
    expect!(Some(6)).to(be_some());
}

#[test]
fn to_be_some_value() {
    expect!(Some(9)).to(be_some().value(9));
}

#[test]
#[should_panic]
fn to_be_some_should_panic() {
    expect!(None::<u8>).to(be_some());
}

#[test]
#[should_panic]
fn to_be_some_value_should_panic() {
    expect!(None::<u8>).to(be_some().value(9));
}

#[test]
fn to_be_none() {
    expect!(None::<u8>).to(be_none());
}

#[test]
#[should_panic]
fn to_be_none_should_panic() {
    expect!(Some(4)).to(be_none());
}

#[test]
fn to_be_some_for_type_without_partial_eq() {
    #[derive(Debug)]
    struct Foo;
    expect!(Some(Foo)).to(be_some());
}

#[test]
fn to_be_some_value_for_type_with_partial_eq() {
    #[derive(Debug, PartialEq)]
    struct Foo;
    expect!(Some(Foo)).to(be_some().value(Foo));
}
