// Answer 0

#[test]
fn byte_offset_with_some_ch() {
    struct MockIterator {
        offset: usize,
    }

    impl MockIterator {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    struct MockRead {
        iter: MockIterator,
        ch: Option<u8>,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.ch)
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            match self.ch {
                Some(_) => self.iter.byte_offset() - 1,
                None => self.iter.byte_offset(),
            }
        }
        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead {
        iter: MockIterator { offset: 5 },
        ch: Some(1),
    };
    assert_eq!(read.byte_offset(), 4);
}

#[test]
fn byte_offset_with_none_ch() {
    struct MockIterator {
        offset: usize,
    }

    impl MockIterator {
        fn byte_offset(&self) -> usize {
            self.offset
        }
    }

    struct MockRead {
        iter: MockIterator,
        ch: Option<u8>,
    }

    impl Read<'static> for MockRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(self.ch)
        }
        fn discard(&mut self) {}
        fn position(&self) -> Position {
            Position::default()
        }
        fn peek_position(&self) -> Position {
            Position::default()
        }
        fn byte_offset(&self) -> usize {
            match self.ch {
                Some(_) => self.iter.byte_offset() - 1,
                None => self.iter.byte_offset(),
            }
        }
        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut read = MockRead {
        iter: MockIterator { offset: 10 },
        ch: None,
    };
    assert_eq!(read.byte_offset(), 10);
}

