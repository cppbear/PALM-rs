// Answer 0

#[derive(Debug)]
enum Category {
    Io,
    Syntax,
    Data,
    Eof,
}

#[derive(Debug)]
enum ErrorCode {
    Message(String),
    Io(String),
    EofWhileParsingList,
    EofWhileParsingObject,
    EofWhileParsingString,
    EofWhileParsingValue,
    ExpectedColon,
    ExpectedListCommaOrEnd,
    // More variants omitted for brevity
}

struct Error {
    code: ErrorCode,
}

struct JsonError {
    err: Error,
}

impl JsonError {
    pub fn classify(&self) -> Category {
        match self.err.code {
            ErrorCode::Message(_) => Category::Data,
            ErrorCode::Io(_) => Category::Io,
            ErrorCode::EofWhileParsingList
            | ErrorCode::EofWhileParsingObject
            | ErrorCode::EofWhileParsingString
            | ErrorCode::EofWhileParsingValue => Category::Eof,
            _ => Category::Syntax,
        }
    }
}

#[test]
fn test_classify_eof_while_parsing_list() {
    let error = JsonError {
        err: Error {
            code: ErrorCode::EofWhileParsingList,
        },
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_object() {
    let error = JsonError {
        err: Error {
            code: ErrorCode::EofWhileParsingObject,
        },
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_string() {
    let error = JsonError {
        err: Error {
            code: ErrorCode::EofWhileParsingString,
        },
    };
    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_value() {
    let error = JsonError {
        err: Error {
            code: ErrorCode::EofWhileParsingValue,
        },
    };
    assert_eq!(error.classify(), Category::Eof);
}

