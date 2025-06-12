// Answer 0

#[test]
fn test_fmt_non_zero_line() {
    struct ErrorStruct {
        line: usize,
        column: usize,
        code: String,
    }

    impl std::fmt::Display for ErrorStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.line == 0 {
                write!(f, "{}", self.code)
            } else {
                write!(
                    f,
                    "{} at line {} column {}",
                    self.code, self.line, self.column
                )
            }
        }
    }

    let error = ErrorStruct {
        line: 5,
        column: 10,
        code: String::from("Error occurrence"),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, "Error occurrence at line 5 column 10");
}

#[test]
#[should_panic]
fn test_fmt_panic_on_unreachable_code() {
    struct PanicStruct {
        line: usize,
        column: usize,
        code: String,
    }

    impl std::fmt::Display for PanicStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // This condition should never occur in this context and is for panic simulation
            if self.line == 0 {
                panic!("Should not reach this point for line = 0");
            } else {
                write!(
                    f,
                    "{} at line {} column {}",
                    self.code, self.line, self.column
                )
            }
        }
    }

    let panic_error = PanicStruct {
        line: 0,
        column: 0,
        code: String::from("This should cause a panic"),
    };

    let _ = format!("{}", panic_error);
}

