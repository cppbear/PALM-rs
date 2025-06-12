// Answer 0

#[test]
fn test_peek_position() {
    struct MockSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> Read<'a> for MockSliceRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.slice.len() {
                let byte = self.slice[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.slice.len() {
                Ok(Some(self.slice[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    struct MockStrRead<'a> {
        delegate: MockSliceRead<'a>,
    }

    impl<'a> Read<'a> for MockStrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            self.delegate.next()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            self.delegate.peek()
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.delegate.position()
        }

        fn peek_position(&self) -> Position {
            self.delegate.peek_position()
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
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let data: &[u8] = b"Hello, world!";
    let mock_slice_read = MockSliceRead { slice: data, index: 0 };
    let mut str_read = MockStrRead { delegate: mock_slice_read };

    let position = str_read.peek_position();
    assert_eq!(position.line, 0);
    assert_eq!(position.column, 0); // Initial position should be at index 0

    // Call `next` and check position again
    str_read.next().unwrap();
    let position_after_next = str_read.peek_position();
    assert_eq!(position_after_next.line, 0);
    assert_eq!(position_after_next.column, 1); // After one byte read, column should be 1
}

