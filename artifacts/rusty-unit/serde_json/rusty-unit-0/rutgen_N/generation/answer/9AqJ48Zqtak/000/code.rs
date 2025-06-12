// Answer 0

#[test]
fn test_fmt() {
    use std::fmt::{self, Display};

    struct TestError {
        err: String,
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.err)
        }
    }

    let test_error = TestError {
        err: String::from("An error occurred"),
    };
    
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", test_error);

    assert!(result.is_ok());
    assert_eq!(buffer, "An error occurred");
}

