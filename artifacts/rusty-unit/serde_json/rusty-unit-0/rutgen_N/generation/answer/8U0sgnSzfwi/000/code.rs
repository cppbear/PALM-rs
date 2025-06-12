// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct Error {
        code: String,
        line: usize,
        column: usize,
    }

    struct MyError {
        err: Error,
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Error({:?}, line: {}, column: {})",
                self.err.code.to_string(),
                self.err.line,
                self.err.column
            )
        }
    }

    let my_error = MyError {
        err: Error {
            code: "404".to_string(),
            line: 10,
            column: 20,
        },
    };

    let result = format!("{}", my_error);
    assert_eq!(result, "Error(\"404\", line: 10, column: 20)");
}

