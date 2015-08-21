
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn to_have_count_3() {
    expect!(vec![1, 2, 3].iter()).to(have_count(3));
}

#[test]
fn to_have_count_0() {
    expect!(Vec::<u32>::new().iter()).to(have_count(0));
}
