#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn equality_of_vecs() {
    expect!(vec![1, 2, 3]).to(be_equal_to([1, 2, 3]));
}

#[test]
fn equality_of_strings() {
    expect!("hello".to_string()).to(be_equal_to("hello"));
}

#[test]
#[should_panic]
fn equality_of_vecs_should_panic() {
    expect!(vec![1, 2, 2]).to(be_equal_to([1, 2, 3]));
}

#[test]
#[should_panic]
fn equality_of_strings_should_panic() {
    expect!("hell0".to_string()).to(be_equal_to("hello"));
}
