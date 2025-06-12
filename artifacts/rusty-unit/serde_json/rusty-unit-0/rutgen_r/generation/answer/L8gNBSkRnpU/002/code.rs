// Answer 0

#[derive(Debug)]
struct Error {
    code: ErrorCode,
    message: String,
}

#[derive(Debug)]
struct ErrorCode;

struct MyStruct {
    err: MyError,
}

struct MyError {
    line: usize,
    code: ErrorCode,
}

impl MyStruct {
    pub(crate) fn fix_position<F>(self, f: F) -> Self
    where
        F: FnOnce(ErrorCode) -> Error,
    {
        if self.err.line == 0 {
            f(self.err.code)
        } else {
            self
        }
    }
}

#[test]
fn test_fix_position_with_line_zero() {
    let error_code = ErrorCode;
    let my_error = MyError {
        line: 0,
        code: error_code,
    };

    let my_struct = MyStruct { err: my_error };

    let result = my_struct.fix_position(|_code| Error {
        code: _code,
        message: "Error occurred".to_string(),
    });

    assert_eq!(result.err.line, 0);
}

#[test]
fn test_fix_position_with_line_non_zero() {
    let error_code = ErrorCode;
    let my_error = MyError {
        line: 1,
        code: error_code,
    };

    let my_struct = MyStruct { err: my_error };

    let result = my_struct.fix_position(|_code| Error {
        code: _code,
        message: "This should not happen".to_string(),
    });

    assert_eq!(result.err.line, 1);
}

