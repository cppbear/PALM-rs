// Answer 0

#[test]
fn test_parse_flags_dangling_negation() {
    struct TestParser {
        input: Vec<char>,
        index: usize,
    }

    impl TestParser {
        fn new(input: &str) -> Self {
            TestParser {
                input: input.chars().collect(),
                index: 0,
            }
        }

        fn char(&self) -> char {
            self.input[self.index]
        }

        fn bump(&mut self) -> bool {
            if self.index < self.input.len() - 1 {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn span(&self) -> Span {
            Span { start: Position { offset: 0, line: 1, column: 1 }, end: Position { offset: 0, line: 1, column: 1 } }
        }

        fn span_char(&self) -> Span {
            self.span()
        }

        fn error(&self, span: Span, kind: ErrorKind) -> Error {
            Error { kind, pattern: self.input.iter().collect(), span }
        }

        fn parse_flag(&self) -> Result<Flag> {
            match self.char() {
                'i' => Ok(Flag::CaseInsensitive),
                'm' => Ok(Flag::MultiLine),
                's' => Ok(Flag::DotMatchesNewLine),
                _ => Err(self.error(self.span_char(), ErrorKind::FlagUnrecognized)),
            }
        }
    }

    impl Borrow<TestParser> for TestParser {
        fn borrow(&self) -> &TestParser {
            self
        }
    }

    let mut parser = TestParser::new("-im)");

    let result = parser.parse_flags();
    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(error.kind, ErrorKind::FlagDanglingNegation);
    }
}

