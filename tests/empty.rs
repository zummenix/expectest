#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn emptiness_of_str() {
    expect!("".chars()).to(be_empty());
}

#[test]
#[should_panic]
fn emptiness_of_str_should_panic() {
    expect!("world".chars()).to(be_empty());
}

#[test]
fn emptiness_of_vec() {
    let v: Vec<u32> = vec![];
    expect!(v.iter()).to(be_empty());
}

#[test]
#[should_panic]
fn emptiness_of_vec_should_panic() {
    let v = vec![1, 2];
    expect!(v.iter()).to(be_empty());
}
