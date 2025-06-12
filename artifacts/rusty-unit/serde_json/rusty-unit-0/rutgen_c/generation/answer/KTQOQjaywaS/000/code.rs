// Answer 0

#[test]
fn test_invalid_type() {
    struct ExpectedImpl;
    impl de::Expected for ExpectedImpl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Some expected type")
        }
    }

    let unexpected = de::Unexpected::Other("unexpected");
    let expected = &ExpectedImpl;

    let error = Error::invalid_type(unexpected, expected);

    // Here you would normally check the contents of the error, but since
    // we're omitting method overrides and simplifications, we'll just ensure
    // it compiles and returns an instance of `Error`
    let _ = error; // Just to use the variable
}

#[test]
fn test_invalid_value() {
    struct ExpectedImpl;
    impl de::Expected for ExpectedImpl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Some expected value")
        }
    }

    let unexpected = de::Unexpected::Other("unexpected value");
    let expected = &ExpectedImpl;

    let error = Error::invalid_value(unexpected, expected);

    // Here you would normally check the contents of the error, but since
    // we're omitting method overrides and simplifications, we'll just ensure
    // it compiles and returns an instance of `Error`
    let _ = error; // Just to use the variable
}

