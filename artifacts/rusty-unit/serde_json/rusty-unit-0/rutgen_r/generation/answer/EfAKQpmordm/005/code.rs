// Answer 0

#[test]
fn test_ignore_str_valid_escape_sequence() {
    struct TestReader<'a> {
        input: &'a [u8],
        index: usize,
    }

    impl<'a> TestReader<'a> {
        fn next_or_eof(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }
    }

    fn is_escape(ch: u8, _: bool) -> bool {
        ch == b'\\' || ch == b'"'
    }

    fn ignore_escape(_: &mut TestReader) -> Result<(), ()> {
        Ok(())
    }
    
    fn error(_: &TestReader, _: ErrorCode) -> Result<(), ()> {
        Err(())
    }

    impl TestReader<'_> {
        fn ignore_str(&mut self) -> Result<(), ()> {
            loop {
                let ch = self.next_or_eof().unwrap();
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        ignore_escape(self)?;
                    }
                    _ => {
                        return error(self, ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut reader = TestReader {
        input: &[b'\\', b'"'],
        index: 0,
    };
    let result = reader.ignore_str();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_ignore_str_only_double_quote() {
    struct TestReader<'a> {
        input: &'a [u8],
        index: usize,
    }

    impl<'a> TestReader<'a> {
        fn next_or_eof(&mut self) -> Result<u8, ()> {
            if self.index < self.input.len() {
                let ch = self.input[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(())
            }
        }
    }

    fn is_escape(ch: u8, _: bool) -> bool {
        ch == b'"'
    }

    fn ignore_escape(_: &mut TestReader) -> Result<(), ()> {
        Ok(())
    }
    
    fn error(_: &TestReader, _: ErrorCode) -> Result<(), ()> {
        Err(())
    }

    impl TestReader<'_> {
        fn ignore_str(&mut self) -> Result<(), ()> {
            loop {
                let ch = self.next_or_eof().unwrap();
                if !is_escape(ch, true) {
                    continue;
                }
                match ch {
                    b'"' => {
                        return Ok(());
                    }
                    b'\\' => {
                        ignore_escape(self)?;
                    }
                    _ => {
                        return error(self, ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut reader = TestReader {
        input: &[b'"'],
        index: 0,
    };
    let result = reader.ignore_str();
    assert_eq!(result, Ok(()));
}

