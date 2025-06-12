// Answer 0

#[test]
fn test_peek_valid_case() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position {
            unimplemented!()
        }
        fn peek_position(&self) -> Position {
            unimplemented!()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let data = vec![1, 2, 3];
    let mut mock_reader = MockReader { data, position: 0 };
    let mut deserializer = Deserializer { read: mock_reader, scratch: vec![], remaining_depth: 0 };
    
    let result = deserializer.peek().unwrap();
    assert_eq!(result, Some(1));
}

#[test]
fn test_peek_no_data() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            unimplemented!()
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position {
            unimplemented!()
        }
        fn peek_position(&self) -> Position {
            unimplemented!()
        }
        fn byte_offset(&self) -> usize {
            self.position
        }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }
        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let data = vec![];
    let mut mock_reader = MockReader { data, position: 0 };
    let mut deserializer = Deserializer { read: mock_reader, scratch: vec![], remaining_depth: 0 };
    
    let result = deserializer.peek().unwrap();
    assert_eq!(result, None);
}

