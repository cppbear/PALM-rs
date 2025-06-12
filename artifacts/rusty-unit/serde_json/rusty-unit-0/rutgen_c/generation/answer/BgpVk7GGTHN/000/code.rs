// Answer 0

#[test]
fn test_parse_str_success() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> Read<'a> for StrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.delegate.index < self.delegate.slice.len() {
                let byte = self.delegate.slice[self.delegate.index];
                self.delegate.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.delegate.index < self.delegate.slice.len() {
                Ok(Some(self.delegate.slice[self.delegate.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.delegate.index = self.delegate.slice.len();
        }

        fn position(&self) -> Position {
            // Mock position
            Position::default()
        }

        fn peek_position(&self) -> Position {
            // Mock position
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.delegate.index
        }
        
        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'a, 's, str>> {
            self.delegate
                .parse_str_bytes(
                    scratch,
                    true,
                    |_, bytes| { Ok(unsafe { str::from_utf8_unchecked(bytes) }) },
                )
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let data = b"hello world";
    let mut slice_read = StrRead {
        delegate: MockSliceRead { slice: data, index: 0 },
        #[cfg(feature = "raw_value")]
        data: std::str::from_utf8(data).unwrap(),
    };

    let mut scratch = Vec::new();
    let result = slice_read.parse_str(&mut scratch);
    assert!(result.is_ok());
    if let Ok(Reference::Borrowed(parsed_str)) = result {
        assert_eq!(parsed_str, "hello world");
    }
}

#[test]
#[should_panic]
fn test_parse_str_invalid_utf8() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> Read<'a> for StrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.delegate.index < self.delegate.slice.len() {
                let byte = self.delegate.slice[self.delegate.index];
                self.delegate.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.delegate.index < self.delegate.slice.len() {
                Ok(Some(self.delegate.slice[self.delegate.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.delegate.index = self.delegate.slice.len();
        }

        fn position(&self) -> Position {
            // Mock position
            Position::default()
        }

        fn peek_position(&self) -> Position {
            // Mock position
            Position::default()
        }

        fn byte_offset(&self) -> usize {
            self.delegate.index
        }
        
        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'a, 's, str>> {
            self.delegate
                .parse_str_bytes(
                    scratch,
                    true,
                    |_, bytes| { Ok(unsafe { str::from_utf8_unchecked(bytes) }) },
                )
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let data = b"\xFF\xFE\xFD"; // Invalid UTF-8 bytes
    let mut slice_read = StrRead {
        delegate: MockSliceRead { slice: data, index: 0 },
        #[cfg(feature = "raw_value")]
        data: std::str::from_utf8(data).unwrap(),
    };

    let mut scratch = Vec::new();
    let _ = slice_read.parse_str(&mut scratch);
}

