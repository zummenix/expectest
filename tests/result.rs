use expectest::prelude::*;

#[test]
fn to_be_ok() {
    expect!("4".parse::<u32>()).to(be_ok());
}

#[test]
fn to_be_ok_value() {
    expect!("4".parse::<u32>()).to(be_ok().value(4));
}

#[test]
#[should_panic]
fn to_be_ok_should_panic() {
    expect!("a".parse::<u32>()).to(be_ok());
}

#[test]
#[should_panic]
fn to_be_ok_value_should_panic() {
    expect!("4".parse::<u32>()).to(be_ok().value(3));
}

#[test]
fn to_be_ok_for_type_without_partial_eq() {
    #[derive(Debug)]
    struct Foo;
    #[derive(Debug)]
    struct Bar;

    fn ok_result(v: Foo) -> Result<Foo, Bar> {
        Ok(v)
    }
    expect!(ok_result(Foo)).to(be_ok());
}

fn err_result(v: u32) -> Result<f32, u32> {
    Err(v)
}

fn ok_result(v: f32) -> Result<f32, u32> {
    Ok(v)
}

#[test]
fn to_be_err() {
    expect!(err_result(9)).to(be_err());
}

#[test]
fn to_be_err_value() {
    expect!(err_result(9)).to(be_err().value(9));
}

#[test]
#[should_panic]
fn to_be_err_should_panic() {
    expect!(ok_result(1.0)).to(be_err());
}

#[test]
#[should_panic]
fn to_be_err_value_should_panic() {
    expect!(ok_result(1.0)).to(be_err().value(9));
}

#[test]
fn to_be_err_for_type_without_partial_eq() {
    #[derive(Debug)]
    struct Foo;
    #[derive(Debug)]
    struct Bar;

    fn err_result(v: Bar) -> Result<Foo, Bar> {
        Err(v)
    }
    expect!(err_result(Bar)).to(be_err());
}
