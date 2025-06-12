// Answer 0

#[test]
fn test_classify_control_character_while_parsing_string() {
    struct ErrorCode {
        code: ErrorCodeVariant,
    }

    enum ErrorCodeVariant {
        ControlCharacterWhileParsingString,
    }

    struct ErrorImpl {
        code: ErrorCode,
        line: usize,
        column: usize,
    }
    
    struct Error {
        err: Box<ErrorImpl>,
    }

    let error_code = ErrorCode {
        code: ErrorCodeVariant::ControlCharacterWhileParsingString,
    };

    let error_impl = ErrorImpl {
        code: error_code,
        line: 1,
        column: 10,
    };

    let error = Error {
        err: Box::new(error_impl),
    };

    assert_eq!(error.classify(), Category::Syntax);
}

