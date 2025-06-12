// Answer 0

#[test]
fn test_next_return_some_value() {
    struct TestReader {
        values: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;
        
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                let value = self.values[self.index];
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            // Implement as necessary
            Ok(None)
        }

        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            // Implement as necessary
            Position::default() 
        }

        fn peek_position(&self) -> Position {
            // Implement as necessary
            Position::default() 
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            // Implement as necessary
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            // Implement as necessary
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            // Implement as necessary
            unimplemented!()
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Implement as necessary
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            // Implement as necessary
            unimplemented!()
        }
    }

    let mut reader = TestReader { values: vec![1, 2, 3], index: 0 };
    assert_eq!(reader.next().unwrap(), Some(1));
    assert_eq!(reader.next().unwrap(), Some(2));
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn test_next_return_none() {
    struct EmptyReader;

    impl Read<'_> for EmptyReader {
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
            0
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut reader = EmptyReader;
    assert_eq!(reader.next().unwrap(), None);
}

