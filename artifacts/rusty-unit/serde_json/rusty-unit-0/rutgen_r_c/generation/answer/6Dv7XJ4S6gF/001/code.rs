// Answer 0

#[test]
fn test_peek_empty_slice() {
    struct TestRead<'a> {
        delegate: SliceRead<'a>,
    }

    impl<'a> TestRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self {
                delegate: SliceRead {
                    slice,
                    index: 0,
                    #[cfg(feature = "raw_value")]
                    raw_buffering_start_index: 0,
                },
            }
        }
    }

    impl<'a> Read<'a> for TestRead<'a> {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.delegate.index)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.delegate.index)
        }

        fn byte_offset(&self) -> usize {
            self.delegate.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'a>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestRead::new(&[]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_peek_non_empty_slice() {
    struct TestRead<'a> {
        delegate: SliceRead<'a>,
    }

    impl<'a> TestRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self {
                delegate: SliceRead {
                    slice,
                    index: 0,
                    #[cfg(feature = "raw_value")]
                    raw_buffering_start_index: 0,
                },
            }
        }
    }
    
    impl<'a> Read<'a> for TestRead<'a> {
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

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.delegate.index)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.delegate.index)
        }

        fn byte_offset(&self) -> usize {
            self.delegate.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'a>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestRead::new(&[1, 2, 3]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), Some(1));
}

