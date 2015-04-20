
#[macro_use(expect)]
extern crate expectest;
use expectest::prelude::*;

#[test]
fn some() {
    expect!(Some(6)).to(be_some());
    expect!(Some(9)).to(be_some().value(9));
}

#[test]
#[should_panic]
fn some_should_panic() {
    expect!(None::<u8>).to(be_some());
    expect!(None::<u8>).to(be_some().value(9));
}
