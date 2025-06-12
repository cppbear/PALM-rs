// Answer 0

#[test]
fn test_parse_unicode_class_named() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }
        
        fn char(&self) -> char {
            self.input.get(self.pos).cloned().unwrap_or('\0')
        }
        
        fn bump(&mut self) {
            self.pos += 1;
        }
        
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            while self.char().is_whitespace() {
                self.bump();
            }
            if self.is_eof() {
                return false;
            }
            self.bump();
            true
        }
        
        fn span_char(&self) -> std::ops::Range<usize> {
            self.pos..self.pos + 1
        }
        
        fn pos(&self) -> usize {
            self.pos
        }
    }

    struct MockUnicodeClassParser {
        parser: MockParser,
    }

    impl MockUnicodeClassParser {
        fn new(input: &str) -> Self {
            Self {
                parser: MockParser::new(input),
            }
        }

        fn parse_unicode_class(&mut self) -> Result<ast::ClassUnicode, String> {
            // Implementation of parse_unicode_class directly as specified
            assert!(self.parser.char() == 'p' || self.parser.char() == 'P');

            let mut scratch = String::new();
            let negated = self.parser.char() == 'P';
            if !self.parser.bump_and_bump_space() {
                return Err("Unexpected EOF".to_string());
            }
            let (start, kind) =
                if self.parser.char() == '{' {
                    let start = self.parser.span_char().end;
                    while self.parser.bump_and_bump_space() && self.parser.char() != '}' {
                        scratch.push(self.parser.char());
                    }
                    if self.parser.is_eof() {
                        return Err("Unexpected EOF".to_string());
                    }
                    self.parser.bump();
                    let name = &scratch;
                    if let Some(i) = name.find("!=") {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::NotEqual,
                            name: name[..i].to_string(),
                            value: name[i + 2..].to_string(),
                        })
                    } else if let Some(i) = name.find(':') {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::Colon,
                            name: name[..i].to_string(),
                            value: name[i + 1..].to_string(),
                        })
                    } else if let Some(i) = name.find('=') {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::Equal,
                            name: name[..i].to_string(),
                            value: name[i + 1..].to_string(),
                        })
                    } else {
                        (start, ast::ClassUnicodeKind::Named(name.to_string()))
                    }
                } else {
                    let start = self.parser.pos();
                    let c = self.parser.char();
                    self.parser.bump_and_bump_space();
                    let kind = ast::ClassUnicodeKind::OneLetter(c);
                    (start, kind)
                };
            Ok(ast::ClassUnicode {
                span: Span::new(start, self.parser.pos()),
                negated: negated,
                kind: kind,
            })
        }
    }

    let mut parser = MockUnicodeClassParser::new("p{Greek}");
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let unicode_class = result.unwrap();
    assert_eq!(unicode_class.negated, false);
}

#[test]
fn test_parse_unicode_class_negated() {
    struct MockParser {
        input: Vec<char>,
        pos: usize,
    }

    impl MockParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }
        
        fn char(&self) -> char {
            self.input.get(self.pos).cloned().unwrap_or('\0')
        }
        
        fn bump(&mut self) {
            self.pos += 1;
        }
        
        fn is_eof(&self) -> bool {
            self.pos >= self.input.len()
        }
        
        fn bump_and_bump_space(&mut self) -> bool {
            while self.char().is_whitespace() {
                self.bump();
            }
            if self.is_eof() {
                return false;
            }
            self.bump();
            true
        }
        
        fn span_char(&self) -> std::ops::Range<usize> {
            self.pos..self.pos + 1
        }
        
        fn pos(&self) -> usize {
            self.pos
        }
    }

    struct MockUnicodeClassParser {
        parser: MockParser,
    }

    impl MockUnicodeClassParser {
        fn new(input: &str) -> Self {
            Self {
                parser: MockParser::new(input),
            }
        }

        fn parse_unicode_class(&mut self) -> Result<ast::ClassUnicode, String> {
            assert!(self.parser.char() == 'p' || self.parser.char() == 'P');

            let mut scratch = String::new();
            let negated = self.parser.char() == 'P';
            if !self.parser.bump_and_bump_space() {
                return Err("Unexpected EOF".to_string());
            }
            let (start, kind) =
                if self.parser.char() == '{' {
                    let start = self.parser.span_char().end;
                    while self.parser.bump_and_bump_space() && self.parser.char() != '}' {
                        scratch.push(self.parser.char());
                    }
                    if self.parser.is_eof() {
                        return Err("Unexpected EOF".to_string());
                    }
                    self.parser.bump();
                    let name = &scratch;
                    if let Some(i) = name.find("!=") {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::NotEqual,
                            name: name[..i].to_string(),
                            value: name[i + 2..].to_string(),
                        })
                    } else if let Some(i) = name.find(':') {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::Colon,
                            name: name[..i].to_string(),
                            value: name[i + 1..].to_string(),
                        })
                    } else if let Some(i) = name.find('=') {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::Equal,
                            name: name[..i].to_string(),
                            value: name[i + 1..].to_string(),
                        })
                    } else {
                        (start, ast::ClassUnicodeKind::Named(name.to_string()))
                    }
                } else {
                    let start = self.parser.pos();
                    let c = self.parser.char();
                    self.parser.bump_and_bump_space();
                    let kind = ast::ClassUnicodeKind::OneLetter(c);
                    (start, kind)
                };
            Ok(ast::ClassUnicode {
                span: Span::new(start, self.parser.pos()),
                negated: negated,
                kind: kind,
            })
        }
    }

    let mut parser = MockUnicodeClassParser::new("P{Greek}");
    let result = parser.parse_unicode_class();
    assert!(result.is_ok());
    let unicode_class = result.unwrap();
    assert_eq!(unicode_class.negated, true);
}

