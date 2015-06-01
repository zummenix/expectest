
extern crate expectest;

use expectest::core::expect;
use expectest::matchers::be_empty;
use expectest::traits::IsEmpty;

fn main() {
    let a = Empty;
    expect(a).to(be_empty());
}

#[derive(Debug)]
struct Empty;

impl IsEmpty for Empty {
    fn is_empty(&self) -> bool {
        true
    }
}
