// Answer 0

#[test]
fn test_description_non_exhaustive() {
    let error = Error::__Nonexhaustive;
    let _result = error.description();
}

#[should_panic]
#[test]
fn test_description_should_panic_on_non_exhaustive() {
    let error = Error::__Nonexhaustive;
    let _result = error.description();
}

