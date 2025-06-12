// Answer 0

#[test]
fn test_fmt_recursion_limit_exceeded() {
    struct ErrorCode {
        message: String,
    }

    impl std::fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    enum ErrorCodeEnum {
        RecursionLimitExceeded,
        Message(String),
    }

    impl std::fmt::Display for ErrorCodeEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorCodeEnum::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
                ErrorCodeEnum::Message(msg) => f.write_str(msg),
            }
        }
    }

    let error = ErrorCodeEnum::RecursionLimitExceeded;
    let mut output = String::new();
    let result = std::panic::catch_unwind(|| {
        let _ = write!(&mut output, "{}", error);
    });

    assert!(result.is_ok());
    assert_eq!(output, "recursion limit exceeded");
}

