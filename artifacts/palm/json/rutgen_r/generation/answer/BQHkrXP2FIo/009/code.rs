// Answer 0

#[test]
fn test_classify_control_character_while_parsing_string() {
    struct MockError {
        err: MockErr,
    }

    struct MockErr {
        code: ErrorCode,
    }

    enum ErrorCode {
        ControlCharacterWhileParsingString,
    }

    enum Category {
        Io,
        Syntax,
        Data,
        Eof,
    }

    impl MockError {
        pub fn classify(&self) -> Category {
            match self.err.code {
                ErrorCode::ControlCharacterWhileParsingString => Category::Syntax,
            }
        }
    }

    let error = MockError {
        err: MockErr {
            code: ErrorCode::ControlCharacterWhileParsingString,
        },
    };
    
    assert_eq!(error.classify(), Category::Syntax);
}

