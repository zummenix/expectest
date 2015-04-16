
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn expect_macro() {
    expect!(1 + 1).to(equal(2));
}
