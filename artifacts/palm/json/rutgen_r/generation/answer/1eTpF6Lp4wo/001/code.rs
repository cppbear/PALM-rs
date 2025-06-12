// Answer 0

#[test]
fn test_fmt_with_non_zero_line() {
    struct Error {
        code: &'static str,
        line: usize,
        column: usize,
    }
    
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.line == 0 {
                std::fmt::Display::fmt(&self.code, f)
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
        line: 2,
        column: 10,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "Syntax Error at line 2 column 10");
}

#[test]
fn test_fmt_with_zero_column() {
    struct Error {
        code: &'static str,
        line: usize,
        column: usize,
    }
    
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.line == 0 {
                std::fmt::Display::fmt(&self.code, f)
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
        code: "Unexpected Token",
        line: 5,
        column: 0,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "Unexpected Token at line 5 column 0");
}

#[test]
fn test_fmt_with_large_values() {
    struct Error {
        code: &'static str,
        line: usize,
        column: usize,
    }
    
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if self.line == 0 {
                std::fmt::Display::fmt(&self.code, f)
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
        code: "Overflow Error",
        line: usize::MAX,
        column: usize::MAX,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, format!("Overflow Error at line {} column {}", usize::MAX, usize::MAX));
}

