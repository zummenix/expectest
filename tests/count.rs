#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn vec_to_have_count_3() {
    expect!(vec![1, 2, 3].iter()).to(have_count(3));
}

#[test]
fn vec_to_have_count_0() {
    expect!(Vec::<u32>::new().iter()).to(have_count(0));
}

#[test]
#[should_panic]
fn vec_to_have_count_3_should_panic() {
    expect!(vec![1, 2].iter()).to(have_count(3));
}

#[test]
fn string_to_have_count_3() {
    expect!("abc".chars()).to(have_count(3));
}

#[test]
fn string_to_have_count_0() {
    expect!("".chars()).to(have_count(0));
}

#[test]
#[should_panic]
fn string_to_have_count_3_should_panic() {
    expect!("abcd".chars()).to(have_count(3));
}
