
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

#[test]
fn emptiness_of_vec() {
    let v: Vec<u32> = vec![];
    expect!(v).to(be_empty());
}

#[test]
fn emptiness_of_array() {
    let v: &[u32] = &[];
    expect!(v).to(be_empty());
}

#[test]
#[ignore]
#[should_panic]
fn emptiness_of_vec_should_panic() {
    let v = vec![1, 2];
    expect!(v).to(be_empty());
}

#[test]
#[should_panic]
fn emptiness_of_array_should_panic() {
    let v: &[u32] = &[1, 2];
    expect!(v).to(be_empty());
}
