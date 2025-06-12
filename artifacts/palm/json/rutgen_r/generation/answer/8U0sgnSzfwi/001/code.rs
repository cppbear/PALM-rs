// Answer 0

#[test]
fn test_fmt() {
    struct ErrorDetail {
        code: String,
        line: usize,
        column: usize,
    }

    struct Error {
        err: ErrorDetail,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "Error({:?}, line: {}, column: {})",
                self.err.code.to_string(),
                self.err.line,
                self.err.column
            )
        }
    }

    let error = Error {
        err: ErrorDetail {
            code: "E123".to_string(),
            line: 42,
            column: 7,
        },
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "Error(\"E123\", line: 42, column: 7)");

    let error_empty_code = Error {
        err: ErrorDetail {
            code: "".to_string(),
            line: 0,
            column: 0,
        },
    };

    output.clear();
    let result_empty_code = write!(&mut output, "{}", error_empty_code);
    assert!(result_empty_code.is_ok());
    assert_eq!(output, "Error(\"\", line: 0, column: 0)");

    let error_panic = Error {
        err: ErrorDetail {
            code: "E999".to_string(),
            line: usize::MAX,
            column: 0,
        },
    };

    output.clear();
    let result_panic = write!(&mut output, "{}", error_panic);
    assert!(result_panic.is_ok());
    assert_eq!(output, "Error(\"E999\", line: 18446744073709551615, column: 0)"); // MAX usize as u64
}

