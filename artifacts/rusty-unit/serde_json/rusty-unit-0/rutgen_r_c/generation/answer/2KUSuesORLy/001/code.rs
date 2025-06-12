// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> MockSliceRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index + 2 > self.slice.len() {
                return Err(Error::new("Insufficient bytes for hex escape"));
            }
            let hex_str = core::str::from_utf8(&self.slice[self.index..self.index + 2]).map_err(|_| Error::new("Invalid UTF-8"))?;
            self.index += 2;
            u16::from_str_radix(hex_str, 16).map_err(|_| Error::new("Invalid hex value")).map(|n| n as u16)
        }
    }

    struct MockStrRead<'a> {
        delegate: MockSliceRead<'a>,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) { }
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.delegate.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.delegate.decode_hex_escape()
        }
        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut mock_read = MockStrRead {
        delegate: MockSliceRead::new(b"1A"),
    };

    let result = mock_read.decode_hex_escape().unwrap();
    assert_eq!(result, 0x1A);
}

#[test]
fn test_decode_hex_escape_invalid_length() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> MockSliceRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index + 2 > self.slice.len() {
                return Err(Error::new("Insufficient bytes for hex escape"));
            }
            let hex_str = core::str::from_utf8(&self.slice[self.index..self.index + 2]).map_err(|_| Error::new("Invalid UTF-8"))?;
            self.index += 2;
            u16::from_str_radix(hex_str, 16).map_err(|_| Error::new("Invalid hex value")).map(|n| n as u16)
        }
    }

    struct MockStrRead<'a> {
        delegate: MockSliceRead<'a>,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) { }
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.delegate.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.delegate.decode_hex_escape()
        }
        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut mock_read = MockStrRead {
        delegate: MockSliceRead::new(b"1"),
    };

    let result = mock_read.decode_hex_escape();
    assert!(result.is_err());
}

#[test]
fn test_decode_hex_escape_invalid_hex() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> MockSliceRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index + 2 > self.slice.len() {
                return Err(Error::new("Insufficient bytes for hex escape"));
            }
            let hex_str = core::str::from_utf8(&self.slice[self.index..self.index + 2]).map_err(|_| Error::new("Invalid UTF-8"))?;
            self.index += 2;
            u16::from_str_radix(hex_str, 16).map_err(|_| Error::new("Invalid hex value")).map(|n| n as u16)
        }
    }

    struct MockStrRead<'a> {
        delegate: MockSliceRead<'a>,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
        const should_early_return_if_failed: bool = false;
        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) { }
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.delegate.index }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.delegate.decode_hex_escape()
        }
        fn set_failed(&mut self, _failed: &mut bool) { }
    }

    let mut mock_read = MockStrRead {
        delegate: MockSliceRead::new(b"1G"),
    };

    let result = mock_read.decode_hex_escape();
    assert!(result.is_err());
}

