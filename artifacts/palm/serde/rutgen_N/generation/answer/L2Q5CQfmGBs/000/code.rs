// Answer 0

#[derive(Debug)]
struct Error {
    err: Box<str>,
}

use std::fmt::Display;

impl Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error {
            err: msg.to_string().into_boxed_str(),
        }
    }
}

#[test]
fn test_custom_with_str() {
    let error = Error::custom("An error occurred");
    assert_eq!(&*error.err, "An error occurred");
}

#[test]
fn test_custom_with_numeric() {
    let error = Error::custom(404);
    assert_eq!(&*error.err, "404");
}

#[test]
fn test_custom_with_struct() {
    struct TestStruct;
    impl Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestStruct error")
        }
    }

    let error = Error::custom(TestStruct);
    assert_eq!(&*error.err, "TestStruct error");
}

#[should_panic]
#[test]
fn test_custom_with_empty_string() {
    let error = Error::custom("");
    assert_eq!(&*error.err, "Not an empty string");
}

