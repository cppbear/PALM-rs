// Answer 0

#[derive(Debug)]
struct Parser {
    input: Vec<char>,
    position: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn char(&self) -> char {
        if self.position < self.input.len() {
            self.input[self.position]
        } else {
            '\0' // or some indication of end
        }
    }

    fn bump(&mut self) -> bool {
        if self.position < self.input.len() {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn span_char(&self) -> usize {
        self.position
    }

    // Assuming flags and error management methods are defined or stubbed
    fn error(&self, _span: usize, _kind: ast::ErrorKind) -> ast::Result<ast::Flags> {
        Err(ast::Error { /* fill fields here */ })
    }

    fn parse_flags(&mut self) -> Result<ast::Flags> {
        let mut flags = ast::Flags {
            span: self.span(),
            items: vec![],
        };
        let mut last_was_negation = None;
        while self.char() != ':' && self.char() != ')' {
            if self.char() == '-' {
                last_was_negation = Some(self.span_char());
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Negation,
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::FlagRepeatedNegation {
                            original: flags.items[i].span,
                        },
                    ));
                }
            } else {
                last_was_negation = None;
                let item = ast::FlagsItem {
                    span: self.span_char(),
                    kind: ast::FlagsItemKind::Flag(self.parse_flag()?),
                };
                if let Some(i) = flags.add_item(item) {
                    return Err(self.error(
                        self.span_char(),
                        ast::ErrorKind::FlagDuplicate {
                            original: flags.items[i].span,
                        },
                    ));
                }
            }
            if !self.bump() {
                return Err(self.error(
                    self.span(),
                    ast::ErrorKind::FlagUnexpectedEof,
                ));
            }
        }
        if let Some(span) = last_was_negation {
            return Err(self.error(span, ast::ErrorKind::FlagDanglingNegation));
        }
        flags.span.end = self.pos();
        Ok(flags)
    }
}

#[test]
fn test_parse_flags_dangling_negation() {
    let mut parser = Parser::new("-:"); // Input to trigger the dangling negation scenario
    let result = parser.parse_flags();
    assert!(result.is_err()); // Expecting an error
}

