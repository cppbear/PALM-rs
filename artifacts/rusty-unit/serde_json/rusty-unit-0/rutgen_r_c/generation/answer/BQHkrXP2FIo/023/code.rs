// Answer 0

#[test]
fn test_classify_eof_while_parsing_list() {
    struct ErrorCode {
        kind: String,
    }

    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn classify(&self) -> Category {
            match self.err.code.kind.as_str() {
                "Message" => Category::Data,
                "Io" => Category::Io,
                "EofWhileParsingList" | "EofWhileParsingObject" | "EofWhileParsingString" | "EofWhileParsingValue" => Category::Eof,
                _ => Category::Syntax,
            }
        }
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode { kind: "EofWhileParsingList".to_string() },
            line: 1,
            column: 5,
        }),
    };

    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_object() {
    struct ErrorCode {
        kind: String,
    }

    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn classify(&self) -> Category {
            match self.err.code.kind.as_str() {
                "Message" => Category::Data,
                "Io" => Category::Io,
                "EofWhileParsingList" | "EofWhileParsingObject" | "EofWhileParsingString" | "EofWhileParsingValue" => Category::Eof,
                _ => Category::Syntax,
            }
        }
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode { kind: "EofWhileParsingObject".to_string() },
            line: 1,
            column: 5,
        }),
    };

    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_string() {
    struct ErrorCode {
        kind: String,
    }

    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn classify(&self) -> Category {
            match self.err.code.kind.as_str() {
                "Message" => Category::Data,
                "Io" => Category::Io,
                "EofWhileParsingList" | "EofWhileParsingObject" | "EofWhileParsingString" | "EofWhileParsingValue" => Category::Eof,
                _ => Category::Syntax,
            }
        }
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode { kind: "EofWhileParsingString".to_string() },
            line: 1,
            column: 5,
        }),
    };

    assert_eq!(error.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_value() {
    struct ErrorCode {
        kind: String,
    }

    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }

    struct Error {
        err: Box<ErrorImpl>,
    }

    impl Error {
        pub fn classify(&self) -> Category {
            match self.err.code.kind.as_str() {
                "Message" => Category::Data,
                "Io" => Category::Io,
                "EofWhileParsingList" | "EofWhileParsingObject" | "EofWhileParsingString" | "EofWhileParsingValue" => Category::Eof,
                _ => Category::Syntax,
            }
        }
    }

    let error = Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode { kind: "EofWhileParsingValue".to_string() },
            line: 1,
            column: 5,
        }),
    };

    assert_eq!(error.classify(), Category::Eof);
}

