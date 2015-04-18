
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn bool() {
    expect!(9 == 9).to(be_true());
    expect!(9 == 6).to(be_false());
}
