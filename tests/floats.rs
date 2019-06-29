use expectest::prelude::*;

#[test]
fn equality_of_floats() {
    expect!(12.001_f64).to(be_close_to(12.0));
}

#[test]
fn equality_of_floats_with_delta() {
    expect!(12.1_f64).to(be_close_to(12.0).delta(0.1));
}

#[test]
#[should_panic]
fn equality_of_floats_should_panic() {
    expect!(12.0011_f64).to(be_close_to(12.0));
}

#[test]
#[should_panic]
fn equality_of_floats_with_delta_should_panic() {
    expect!(2.11_f64).to(be_close_to(2.0).delta(0.1));
}
