
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn expect_macro() {
    expect!(1 + 1).to(equal(2));
    expect!(vec![1, 2, 3]).to(equal([1, 2, 3]));
    expect!("hello".to_string()).to(equal("hello"));
}
