#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn in_range() {
    expect!(0).to(be_in_range(0..1));
    expect!(0).to(be_in_range(-1..=0));
    expect!(0).to(be_in_range(-1..));
    expect!(0).to(be_in_range(..1));
    expect!(0).to(be_in_range(..));
    expect!(0.0).to(be_in_range(0.0..1.0));
    expect!(0.0).to(be_in_range(-0.1..=0.0));
}

#[test]
#[should_panic]
fn in_range_less_than_the_lower_bound() {
    expect!(0).to(be_in_range(1..));
}

#[test]
#[should_panic]
fn in_range_equal_to_the_upper_bound_of_exclusive_range() {
    expect!(0).to(be_in_range(-1..0));
}

#[test]
#[should_panic]
fn in_range_less_than_the_lower_exclusive() {
    let range = (std::ops::Bound::Excluded(1), std::ops::Bound::Included(2));
    expect!(0).to(be_in_range(range));
}

#[test]
#[should_panic]
fn in_range_greater_than_the_upper_bound_of_inclusive_range() {
    expect!(0).to(be_in_range(-2..=-1));
}

#[test]
fn in_range_float_infinity() {
    expect!(std::f64::INFINITY).to(be_in_range(0.0..=std::f64::INFINITY));
    expect!(std::f64::INFINITY).to(be_in_range(0.0..));
    expect!(std::f64::NEG_INFINITY).to(be_in_range(std::f64::NEG_INFINITY..0.0));
    expect!(std::f64::NEG_INFINITY).to(be_in_range(..0.0));
}

#[test]
#[should_panic]
fn in_range_float_nan() {
    expect!(std::f64::NAN).to(be_in_range(..));
}
