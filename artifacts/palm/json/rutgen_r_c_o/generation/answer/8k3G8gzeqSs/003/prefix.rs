// Answer 0

#[test]
fn test_peek_with_none_ch_and_some_ok() {
    use alloc::vec;

    struct MockReader {
        next_call: usize,
        history: Vec<Result<Option<u8>>>,
    }

    impl io::Read for MockReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Ok(0)
        }
    }

    impl Read<'static> for IoRead<MockReader> {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.iter.next_call < self.history.len() {
                self.iter.next_call += 1;
                return self.history[self.iter.next_call - 1].clone();
            }
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            match self.ch {
                Some(ch) => Ok(Some(ch)),
                None => match self.next() {
                    Ok(Some(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    Ok(None) => Ok(None),
                    Err(err) => Err(Error::io(err)),
                },
            }
        }

        fn discard(&mut self) {}
        fn position(&self) {}
        fn peek_position(&self) {}
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_reader = MockReader { 
        next_call: 0, 
        history: vec![Ok(Some(42)), Err(io::Error::new(io::ErrorKind::Other, "Error"))],
    };
    
    let mut io_reader = IoRead {
        iter: LineColIterator { iter: mock_reader, line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    
    let _ = io_reader.peek();
}

#[test]
fn test_peek_with_none_ch_and_some_err() {
    use alloc::vec;

    struct MockReader {
        next_call: usize,
        history: Vec<Result<Option<u8>>>,
    }

    impl io::Read for MockReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Ok(0)
        }
    }

    impl Read<'static> for IoRead<MockReader> {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.iter.next_call < self.history.len() {
                self.iter.next_call += 1;
                return self.history[self.iter.next_call - 1].clone();
            }
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            match self.ch {
                Some(ch) => Ok(Some(ch)),
                None => match self.next() {
                    Ok(Some(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    Ok(None) => Ok(None),
                    Err(err) => Err(Error::io(err)),
                },
            }
        }

        fn discard(&mut self) {}
        fn position(&self) {}
        fn peek_position(&self) {}
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_reader = MockReader { 
        next_call: 0, 
        history: vec![Err(io::Error::new(io::ErrorKind::Other, "Error"))],
    };
    
    let mut io_reader = IoRead {
        iter: LineColIterator { iter: mock_reader, line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    
    let _ = io_reader.peek();
}

#[test]
fn test_peek_with_none_ch_and_none() {
    use alloc::vec;

    struct MockReader {
        next_call: usize,
        history: Vec<Result<Option<u8>>>,
    }

    impl io::Read for MockReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Ok(0)
        }
    }

    impl Read<'static> for IoRead<MockReader> {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.iter.next_call < self.history.len() {
                self.iter.next_call += 1;
                return self.history[self.iter.next_call - 1].clone();
            }
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            match self.ch {
                Some(ch) => Ok(Some(ch)),
                None => match self.next() {
                    Ok(Some(ch)) => {
                        self.ch = Some(ch);
                        Ok(self.ch)
                    }
                    Ok(None) => Ok(None),
                    Err(err) => Err(Error::io(err)),
                },
            }
        }

        fn discard(&mut self) {}
        fn position(&self) {}
        fn peek_position(&self) {}
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_reader = MockReader { 
        next_call: 0, 
        history: vec![],
    };
    
    let mut io_reader = IoRead {
        iter: LineColIterator { iter: mock_reader, line: 0, col: 0, start_of_line: 0 },
        ch: None,
        raw_buffer: None,
    };
    
    let _ = io_reader.peek();
}

