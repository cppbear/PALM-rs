// Answer 0

#[test]
fn test_fmt_with_line() {
    struct Error {
        code: &'static str,
        line: usize,
        column: usize,
    }

    impl std::fmt::Display for Error {
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

    let error = Error {
        code: "Syntax Error",
        line: 10,
        column: 5,
    };

    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buf, "Syntax Error at line 10 column 5");
}

#[test]
fn test_fmt_without_line() {
    struct Error {
        code: &'static str,
        line: usize,
        column: usize,
    }

    impl std::fmt::Display for Error {
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

    let error = Error {
        code: "Missing Semicolon",
        line: 0,
        column: 0,
    };

    let mut buf = String::new();
    let result = write!(&mut buf, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buf, "Missing Semicolon");
}

