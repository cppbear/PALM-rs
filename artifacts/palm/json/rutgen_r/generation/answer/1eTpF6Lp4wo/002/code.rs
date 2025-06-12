// Answer 0

#[test]
fn test_fmt_line_zero() {
    struct Error {
        line: usize,
        column: usize,
        code: String,
    }

    use std::fmt;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.line == 0 {
                fmt::Display::fmt(&self.code, f)
            } else {
                write!(
                    f,
                    "{} at line {} column {}",
                    self.code, self.line, self.column
                )
            }
        }
    }

    // Test when line is 0
    let error = Error {
        line: 0,
        column: 5,
        code: String::from("Error code example"),
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "Error code example");
} 

#[test]
fn test_fmt_line_zero_with_empty_code() {
    struct Error {
        line: usize,
        column: usize,
        code: String,
    }

    use std::fmt;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.line == 0 {
                fmt::Display::fmt(&self.code, f)
            } else {
                write!(
                    f,
                    "{} at line {} column {}",
                    self.code, self.line, self.column
                )
            }
        }
    }

    // Test when line is 0 and code is empty
    let error = Error {
        line: 0,
        column: 5,
        code: String::new(),
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "");
}

