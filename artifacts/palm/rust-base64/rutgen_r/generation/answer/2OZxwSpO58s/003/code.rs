// Answer 0

#[test]
fn test_fmt_invalid_length() {
    struct CustomError {
        len: usize,
    }

    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid input length: {}", self.len)
        }
    }

    let error = CustomError { len: 0 };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 0");
}

#[test]
fn test_fmt_invalid_length_large() {
    struct CustomError {
        len: usize,
    }

    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid input length: {}", self.len)
        }
    }

    let error = CustomError { len: 1000 };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Invalid input length: 1000");
}

