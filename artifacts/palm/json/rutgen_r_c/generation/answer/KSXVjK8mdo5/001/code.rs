// Answer 0

#[test]
fn test_byte_offset_for_str_read() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> MockSliceRead<'a> {
        fn byte_offset(&self) -> usize {
            self.index
        }
    }

    struct MockStrRead<'a> {
        delegate: MockSliceRead<'a>,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.delegate.byte_offset()
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
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

    let mock_slice = MockSliceRead {
        slice: b"json_string",
        index: 5,
    };
    
    let mut str_read = MockStrRead {
        delegate: mock_slice,
    };

    assert_eq!(str_read.byte_offset(), 5);
}

#[test]
fn test_byte_offset_at_zero() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> MockSliceRead<'a> {
        fn byte_offset(&self) -> usize {
            self.index
        }
    }

    struct MockStrRead<'a> {
        delegate: MockSliceRead<'a>,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::default()
        }

        fn peek_position(&self) -> Position {
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.delegate.byte_offset()
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
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

    let mock_slice = MockSliceRead {
        slice: b"json_string",
        index: 0,
    };
    
    let mut str_read = MockStrRead {
        delegate: mock_slice,
    };

    assert_eq!(str_read.byte_offset(), 0);
}

