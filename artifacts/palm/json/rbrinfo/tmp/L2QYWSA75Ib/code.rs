fn ignore_value(&mut self) -> Result<()> {
        self.scratch.clear();
        let mut enclosing = None;

        loop {
            let peek = match tri!(self.parse_whitespace()) {
                Some(b) => b,
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
                }
            };

            let frame = match peek {
                b'n' => {
                    self.eat_char();
                    tri!(self.parse_ident(b"ull"));
                    None
                }
                b't' => {
                    self.eat_char();
                    tri!(self.parse_ident(b"rue"));
                    None
                }
                b'f' => {
                    self.eat_char();
                    tri!(self.parse_ident(b"alse"));
                    None
                }
                b'-' => {
                    self.eat_char();
                    tri!(self.ignore_integer());
                    None
                }
                b'0'..=b'9' => {
                    tri!(self.ignore_integer());
                    None
                }
                b'"' => {
                    self.eat_char();
                    tri!(self.read.ignore_str());
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
                match tri!(self.parse_whitespace()) {
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
                match tri!(self.parse_whitespace()) {
                    Some(b'"') => self.eat_char(),
                    Some(_) => return Err(self.peek_error(ErrorCode::KeyMustBeAString)),
                    None => return Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
                }
                tri!(self.read.ignore_str());
                match tri!(self.parse_whitespace()) {
                    Some(b':') => self.eat_char(),
                    Some(_) => return Err(self.peek_error(ErrorCode::ExpectedColon)),
                    None => return Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
                }
            }

            enclosing = Some(frame);
        }
    }