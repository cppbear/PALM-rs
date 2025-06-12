// Answer 0

#[test]
fn test_classify_eof_while_parsing_string() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingString,
            line: 1,
            column: 1,
        }),
    };
    let _ = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_value() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingValue,
            line: 2,
            column: 1,
        }),
    };
    let _ = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_object() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingObject,
            line: 3,
            column: 1,
        }),
    };
    let _ = error.classify();
}

#[test]
fn test_classify_eof_while_parsing_list() {
    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::EofWhileParsingList,
            line: 4,
            column: 1,
        }),
    };
    let _ = error.classify();
}

