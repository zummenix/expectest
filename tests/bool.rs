use expectest::prelude::*;

#[test]
fn to_be_true() {
    expect!(9 == 9).to(be_true());
}

#[test]
fn to_be_false() {
    expect!(9 == 6).to(be_false());
}

#[test]
fn to_be_true_negation() {
    expect!(9 == 6).not_to(be_true());
}

#[test]
fn to_be_false_negation() {
    expect!(9 == 9).to_not(be_false());
}

#[test]
#[should_panic]
fn to_be_true_should_panic() {
    expect!(0 == 1).to(be_true());
}

#[test]
#[should_panic]
fn to_be_false_should_panic() {
    expect!(0 == 0).to(be_false());
}
