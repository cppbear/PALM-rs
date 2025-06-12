// Answer 0

#[test]
fn test_parse_group_non_capturing_flags() {
    struct MockParser {
        pos: Position,
        pattern: String,
        flags: ast::Flags,
        eof: bool,
        lookaround: bool,
        capture_index: u32,
    }

    impl MockParser {
        fn char(&self) -> char {
            '('
        }

        fn span_char(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn bump(&mut self) {
            self.pos.offset += 1;
        }

        fn is_lookaround_prefix(&self) -> bool {
            self.lookaround
        }

        fn bump_if(&mut self, s: &str) -> bool {
            if s == "?" {
                true
            } else {
                false
            }
        }

        fn is_eof(&self) -> bool {
            self.eof
        }

        fn parse_flags(&self) -> Result<ast::Flags> {
            Ok(self.flags.clone())
        }

        fn next_capture_index(&self, _: Span) -> Result<u32> {
            Ok(self.capture_index)
        }

        fn error(&self, _: Span, _: ast::ErrorKind) -> Error {
            Error { kind: ast::ErrorKind::GroupUnclosed, pattern: self.pattern.clone(), span: self.span_char() }
        }

        fn pos(&self) -> Position {
            self.pos
        }

        fn span(&self) -> Span {
            self.span_char()
        }
    }

    let initial_pos = Position { offset: 0, line: 1, column: 1 };
    let flags = ast::Flags {
        span: Span::new(initial_pos, initial_pos),
        items: vec![ast::FlagsItem { span: Span::new(initial_pos, initial_pos), kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) }],
    };

    let mut parser = MockParser {
        pos: initial_pos,
        pattern: "(?i)".to_string(),
        flags,
        eof: false,
        lookaround: false,
        capture_index: 0,
    };

    let result = parser.parse_group();

    assert!(result.is_ok());
    if let Ok(Either::Right(group)) = result {
        assert_eq!(group.kind, ast::GroupKind::NonCapturing(parser.flags.clone()));
        assert_eq!(group.ast, Box::new(Ast::Empty(parser.span())));
    }
}

