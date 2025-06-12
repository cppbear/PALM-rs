// Answer 0

#[test]
fn test_parse_unicode_class_single_character() {
    struct MockParser {
        pos: Position,
        chars: Vec<char>,
        index: usize,
        scratch: RefCell<String>,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                chars,
                index: 0,
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump(&mut self) {
            self.index += 1;
            self.pos.offset += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index < self.chars.len() {
                while self.index < self.chars.len() && self.chars[self.index].is_whitespace() {
                    self.bump();
                }
                return self.index < self.chars.len();
            }
            false
        }

        fn char(&self) -> char {
            if self.index < self.chars.len() {
                self.chars[self.index]
            } else {
                '\0'
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: "".to_string(),
                span: self.span(),
            }
        }

        fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {
            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();

            let negated = self.char() == 'P';
            if !self.bump_and_bump_space() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            let (start, kind) =
                if self.char() == '{' {
                    let start = self.span().end;
                    while self.bump_and_bump_space() && self.char() != '}' {
                        scratch.push(self.char());
                    }
                    if self.is_eof() {
                        return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
                    }
                    assert_eq!(self.char(), '}');
                    self.bump();

                    let name = scratch.as_str();
                    if let Some(i) = name.find("!=") {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::NotEqual,
                            name: name[..i].to_string(),
                            value: name[i + 2..].to_string(),
                        })
                    } else {
                        (start, ast::ClassUnicodeKind::Named(name.to_string()))
                    }
                } else {
                    let start = self.pos;
                    let c = self.char();
                    self.bump_and_bump_space();
                    let kind = ast::ClassUnicodeKind::OneLetter(c);
                    (start, kind)
                };
            Ok(ast::ClassUnicode {
                span: Span::new(start, self.pos),
                negated: negated,
                kind: kind,
            })
        }
    }

    let parser = MockParser::new(vec!['p', '{', 'G', 'r', 'e', 'e', 'k', '}', ' ']);
    let result = parser.parse_unicode_class();

    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_class_multi_character() {
    struct MockParser {
        pos: Position,
        chars: Vec<char>,
        index: usize,
        scratch: RefCell<String>,
    }

    impl MockParser {
        fn new(chars: Vec<char>) -> Self {
            Self {
                pos: Position { offset: 0, line: 1, column: 1 },
                chars,
                index: 0,
                scratch: RefCell::new(String::new()),
            }
        }

        fn bump(&mut self) {
            self.index += 1;
            self.pos.offset += 1;
        }

        fn bump_and_bump_space(&mut self) -> bool {
            if self.index < self.chars.len() {
                while self.index < self.chars.len() && self.chars[self.index].is_whitespace() {
                    self.bump();
                }
                return self.index < self.chars.len();
            }
            false
        }

        fn char(&self) -> char {
            if self.index < self.chars.len() {
                self.chars[self.index]
            } else {
                '\0'
            }
        }

        fn is_eof(&self) -> bool {
            self.index >= self.chars.len()
        }

        fn span(&self) -> Span {
            Span::new(self.pos, self.pos)
        }

        fn error(&self, _span: Span, _kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: ast::ErrorKind::EscapeUnexpectedEof,
                pattern: "".to_string(),
                span: self.span(),
            }
        }

        fn parse_unicode_class(&self) -> Result<ast::ClassUnicode> {
            let mut scratch = self.scratch.borrow_mut();
            scratch.clear();

            let negated = self.char() == 'P';
            if !self.bump_and_bump_space() {
                return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
            }
            let (start, kind) =
                if self.char() == '{' {
                    let start = self.span().end;
                    while self.bump_and_bump_space() && self.char() != '}' {
                        scratch.push(self.char());
                    }
                    if self.is_eof() {
                        return Err(self.error(self.span(), ast::ErrorKind::EscapeUnexpectedEof));
                    }
                    assert_eq!(self.char(), '}');
                    self.bump();

                    let name = scratch.as_str();
                    if let Some(i) = name.find("!=") {
                        (start, ast::ClassUnicodeKind::NamedValue {
                            op: ast::ClassUnicodeOpKind::NotEqual,
                            name: name[..i].to_string(),
                            value: name[i + 2..].to_string(),
                        })
                    } else {
                        (start, ast::ClassUnicodeKind::Named(name.to_string()))
                    }
                } else {
                    let start = self.pos;
                    let c = self.char();
                    self.bump_and_bump_space();
                    let kind = ast::ClassUnicodeKind::OneLetter(c);
                    (start, kind)
                };
            Ok(ast::ClassUnicode {
                span: Span::new(start, self.pos),
                negated: negated,
                kind: kind,
            })
        }
    }

    let parser = MockParser::new(vec!['P', '{', 'n', 'e', 'g', 'a', 't', 'e', 'd', '}', ' ']);
    let result = parser.parse_unicode_class();

    assert!(result.is_ok());
}

