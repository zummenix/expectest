
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn expect_macro() {
    expect!(1 + 1).to(be_equal_to(2));
    expect!(vec![1, 2, 3]).to(be_equal_to([1, 2, 3]));
    expect!("hello".to_string()).to(be_equal_to("hello"));
}
