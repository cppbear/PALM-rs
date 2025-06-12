// Answer 0

#[derive(Debug)]
struct Error {
    message: String,
}

#[derive(Debug)]
struct ErrorCode {
    code: usize,
}

#[derive(Debug)]
struct ErrorContext {
    err: ErrorDetail,
}

#[derive(Debug)]
struct ErrorDetail {
    line: usize,
    code: ErrorCode,
}

impl ErrorContext {
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
fn test_fix_position_line_not_zero() {
    let error_code = ErrorCode { code: 100 };
    let error_detail = ErrorDetail { line: 1, code: error_code };
    let error_context = ErrorContext { err: error_detail };

    let fixed_context = error_context.fix_position(|_| Error { message: String::from("Should not be called") });

    assert_eq!(fixed_context.err.line, 1);
}

#[test]
fn test_fix_position_line_not_zero_with_different_code() {
    let error_code = ErrorCode { code: 200 };
    let error_detail = ErrorDetail { line: 2, code: error_code };
    let error_context = ErrorContext { err: error_detail };

    let fixed_context = error_context.fix_position(|code| Error { message: format!("Error code: {}", code.code) });

    assert_eq!(fixed_context.err.line, 2);
}

