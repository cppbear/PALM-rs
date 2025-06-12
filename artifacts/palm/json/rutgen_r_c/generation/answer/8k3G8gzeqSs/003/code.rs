// Answer 0

#[test]
fn test_peek_none_then_next_ok() {
    struct MockRead {
        iter: Vec<u8>,
        cursor: usize,
        ch: Option<u8>,
    }

    impl MockRead {
        fn new(iter: Vec<u8>) -> Self {
            MockRead { iter, cursor: 0, ch: None }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.cursor < self.iter.len() {
                let byte = self.iter[self.cursor];
                self.cursor += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            MockRead::next(self)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if let Some(ch) = self.ch {
                Ok(Some(ch))
            } else {
                match self.next() {
                    Ok(Some(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    res => res,
                }
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0, byte_offset: self.cursor }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.cursor
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_reader = MockRead::new(vec![1, 2, 3]);
    assert_eq!(mock_reader.peek().unwrap(), Some(1));
}

#[test]
fn test_peek_none_then_next_err() {
    struct MockReadErr {
        iter: Vec<Result<u8, io::Error>>,
        cursor: usize,
        ch: Option<u8>,
    }

    impl MockReadErr {
        fn new(iter: Vec<Result<u8, io::Error>>) -> Self {
            MockReadErr { iter, cursor: 0, ch: None }
        }

        fn next(&mut self) -> Result<Option<u8>> {
            if self.cursor < self.iter.len() {
                let result = &self.iter[self.cursor];
                self.cursor += 1;

                match result {
                    Ok(byte) => Ok(Some(*byte)),
                    Err(err) => Err(Error::io(err.clone())),
                }
            } else {
                Ok(None)
            }
        }
    }

    impl Read<'static> for MockReadErr {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            MockReadErr::next(self)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if let Some(ch) = self.ch {
                Ok(Some(ch))
            } else {
                match self.next() {
                    Ok(Some(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    res => res,
                }
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0, byte_offset: self.cursor }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.cursor
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut mock_reader = MockReadErr::new(vec![Ok(1), Err(io::Error::new(io::ErrorKind::Other, "Error"))]);
    assert!(mock_reader.peek().is_err());
}

