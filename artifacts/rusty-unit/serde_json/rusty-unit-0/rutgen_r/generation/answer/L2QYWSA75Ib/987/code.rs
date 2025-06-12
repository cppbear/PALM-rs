// Answer 0

fn test_ignore_value_empty_list() -> Result<()> {
    struct TestParser {
        scratch: Vec<u8>,
    }

    impl TestParser {
        fn new() -> Self {
            Self { scratch: Vec::new() }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.scratch.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.scratch.remove(0)))
            }
        }

        fn eat_char(&mut self) {
            // Simulate eating a character
            if !self.scratch.is_empty() {
                self.scratch.remove(0);
            }
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(ErrorCode::ExpectedSomeValue)
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<()> {
            Err(ErrorCode::ExpectedSomeValue)
        }

        fn read_ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_value(&mut self) -> Result<()> {
            self.scratch.clear();
            let mut enclosing = None;

            loop {
                let peek = match self.parse_whitespace()? {
                    Some(b) => b,
                    None => {
                        return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                    }
                };

                let frame = match peek {
                    b'n' => {
                        self.eat_char();
                        self.parse_ident(b"ull")?;
                        None
                    }
                    b't' => {
                        self.eat_char();
                        self.parse_ident(b"rue")?;
                        None
                    }
                    b'f' => {
                        self.eat_char();
                        self.parse_ident(b"alse")?;
                        None
                    }
                    b'-' => {
                        self.eat_char();
                        self.ignore_integer()?;
                        None
                    }
                    b'0'..=b'9' => {
                        self.ignore_integer()?;
                        None
                    }
                    b'"' => {
                        self.eat_char();
                        self.read_ignore_str()?;
                        None
                    }
                    frame @ (b'[' | b'{') => {
                        self.scratch.extend(enclosing.take());
                        self.eat_char();
                        Some(frame)
                    }
                    _ => return Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
                };

                let (mut accept_comma, mut frame) = match frame {
                    Some(frame) => (false, frame),
                    None => match enclosing.take() {
                        Some(frame) => (true, frame),
                        None => match self.scratch.pop() {
                            Some(frame) => (true, frame),
                            None => return Ok(()),
                        },
                    },
                };

                loop {
                    match self.parse_whitespace()? {
                        Some(b',') if accept_comma => {
                            self.eat_char();
                            break;
                        }
                        Some(b']') if frame == b'[' => {}
                        Some(b'}') if frame == b'{' => {}
                        Some(_) => {
                            if accept_comma {
                                return Err(self.peek_error(match frame {
                                    b'[' => ErrorCode::ExpectedListCommaOrEnd,
                                    b'{' => ErrorCode::ExpectedObjectCommaOrEnd,
                                    _ => unreachable!(),
                                }));
                            } else {
                                break;
                            }
                        }
                        None => {
                            return Err(self.peek_error(match frame {
                                b'[' => ErrorCode::EofWhileParsingList,
                                b'{' => ErrorCode::EofWhileParsingObject,
                                _ => unreachable!(),
                            }));
                        }
                    }

                    self.eat_char();
                    frame = match self.scratch.pop() {
                        Some(frame) => frame,
                        None => return Ok(()),
                    };
                    accept_comma = true;
                }

                if frame == b'{' {
                    match self.parse_whitespace()? {
                        Some(b'"') => self.eat_char(),
                        Some(_) => return Err(self.peek_error(ErrorCode::KeyMustBeAString)),
                        None => return Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
                    }
                    self.read_ignore_str()?;
                    match self.parse_whitespace()? {
                        Some(b':') => self.eat_char(),
                        Some(_) => return Err(self.peek_error(ErrorCode::ExpectedColon)),
                        None => return Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
                    }
                }

                enclosing = Some(frame);
            }
        }
    }

    let mut parser = TestParser::new();
    parser.scratch.push(b'f');
    parser.ignore_value()
}

#[test]
fn test_ignore_value_panic_on_invalid_identifier() {
    struct TestParser {
        scratch: Vec<u8>,
    }

    impl TestParser {
        fn new() -> Self {
            Self { scratch: vec![b'f'] }
        }

        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.scratch.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.scratch.remove(0)))
            }
        }

        fn eat_char(&mut self) {
            if !self.scratch.is_empty() {
                self.scratch.remove(0);
            }
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Err(ErrorCode::ExpectedSomeValue)
        }

        fn peek_error(&self, _code: ErrorCode) -> Result<()> {
            Err(ErrorCode::ExpectedSomeValue)
        }

        fn read_ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_value(&mut self) -> Result<()> {
            self.scratch.clear();
            let mut enclosing = None;

            loop {
                let peek = match self.parse_whitespace()? {
                    Some(b) => b,
                    None => {
                        return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                    }
                };

                let frame = match peek {
                    b'n' => {
                        self.eat_char();
                        self.parse_ident(b"ull")?;
                        None
                    }
                    b't' => {
                        self.eat_char();
                        self.parse_ident(b"rue")?;
                        None
                    }
                    b'f' => {
                        self.eat_char();
                        self.parse_ident(b"alse")?;
                        None
                    }
                    _ => return Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
                };

                let (mut accept_comma, mut frame) = match frame {
                    Some(frame) => (false, frame),
                    None => match enclosing.take() {
                        Some(frame) => (true, frame),
                        None => match self.scratch.pop() {
                            Some(frame) => (true, frame),
                            None => return Ok(()),
                        },
                    },
                };

                loop {
                    match self.parse_whitespace()? {
                        Some(_) => {}
                        None => {
                            return Err(self.peek_error(ErrorCode::EofWhileParsingList));
                        }
                    }

                    self.eat_char();
                    enclosing = Some(frame);
                }
            }
        }
    }

    let mut parser = TestParser::new();
    assert!(parser.ignore_value().is_err());
}

