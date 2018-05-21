#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn less_than() {
    expect!(0).to(be_less_than(1));
}

#[test]
fn less_than_alias() {
    expect!(0).to(be_lt(1));
}

#[test]
#[should_panic]
fn less_than_should_panic() {
    expect!(0).to(be_less_than(0));
}

#[test]
fn less_or_equal_to_zero() {
    expect!(0).to(be_less_or_equal_to(0));
}

#[test]
fn less_or_equal_to_one() {
    expect!(0).to(be_less_or_equal_to(1));
}

#[test]
fn less_or_equal_to_one_alias() {
    expect!(0).to(be_le(1));
}

#[test]
#[should_panic]
fn less_or_equal_to_should_panic() {
    expect!(1).to(be_less_or_equal_to(0));
}

#[test]
fn greater_than() {
    expect!(1).to(be_greater_than(0));
}

#[test]
fn greater_than_alias() {
    expect!(1).to(be_gt(0));
}

#[test]
#[should_panic]
fn greater_than_should_panic() {
    expect!(1).to(be_greater_than(1));
}

#[test]
fn greater_or_equal_to_zero() {
    expect!(0).to(be_greater_or_equal_to(0));
}

#[test]
fn greater_or_equal_to_minus_one() {
    expect!(0).to(be_greater_or_equal_to(-1));
}

#[test]
fn greater_or_equal_to_minus_one_alias() {
    expect!(0).to(be_ge(-1));
}

#[test]
#[should_panic]
fn greater_or_equal_to_should_panic() {
    expect!(0).to(be_greater_or_equal_to(1));
}
