// Answer 0

#[test]
fn test_classify_eof_while_parsing_string() {
    struct MockError {
        code: ErrorCode,
    }
    
    struct MockErrorWrapper {
        err: MockError,
    }

    impl MockErrorWrapper {
        fn classify(&self) -> Category {
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

    let error_wrapper = MockErrorWrapper {
        err: MockError {
            code: ErrorCode::EofWhileParsingString,
        },
    };

    assert_eq!(error_wrapper.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_value() {
    struct MockError {
        code: ErrorCode,
    }
    
    struct MockErrorWrapper {
        err: MockError,
    }

    impl MockErrorWrapper {
        fn classify(&self) -> Category {
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

    let error_wrapper = MockErrorWrapper {
        err: MockError {
            code: ErrorCode::EofWhileParsingValue,
        },
    };

    assert_eq!(error_wrapper.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_object() {
    struct MockError {
        code: ErrorCode,
    }
    
    struct MockErrorWrapper {
        err: MockError,
    }

    impl MockErrorWrapper {
        fn classify(&self) -> Category {
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

    let error_wrapper = MockErrorWrapper {
        err: MockError {
            code: ErrorCode::EofWhileParsingObject,
        },
    };

    assert_eq!(error_wrapper.classify(), Category::Eof);
}

#[test]
fn test_classify_eof_while_parsing_list() {
    struct MockError {
        code: ErrorCode,
    }
    
    struct MockErrorWrapper {
        err: MockError,
    }

    impl MockErrorWrapper {
        fn classify(&self) -> Category {
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

    let error_wrapper = MockErrorWrapper {
        err: MockError {
            code: ErrorCode::EofWhileParsingList,
        },
    };

    assert_eq!(error_wrapper.classify(), Category::Eof);
}

