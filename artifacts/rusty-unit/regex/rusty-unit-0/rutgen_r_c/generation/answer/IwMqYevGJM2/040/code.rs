// Answer 0

#[test]
fn test_parse_unicode_class_negative_eof() {
    struct DummyParser {
        current_char: char,
        is_eof: bool,
    }

    impl DummyParser {
        fn char(&self) -> char {
            self.current_char
        }

        fn bump_and_bump_space(&mut self) -> bool {
            !self.is_eof
        }

        fn is_eof(&self) -> bool {
            self.is_eof
        }

        fn span(&self) -> Span {
            Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } }
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error { kind, pattern: String::from("Dummy pattern"), span }
        }
    }

    let mut parser = DummyParser { current_char: 'p', is_eof: true };
    let parser_i = ParserI { parser: &parser, pattern: "" };

    let result = parser_i.parse_unicode_class();
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::EscapeUnexpectedEof);
    }
}

