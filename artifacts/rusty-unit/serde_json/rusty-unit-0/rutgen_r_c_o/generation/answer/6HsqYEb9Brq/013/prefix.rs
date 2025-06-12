// Answer 0

#[test]
fn test_ignore_integer_leading_non_zero() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { 
            Ok(Reference::new("")) 
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { 
            Ok(Reference::new(&[])) 
        }

        fn ignore_str(&mut self) -> Result<()> { 
            Ok(()) 
        }

        fn decode_hex_escape(&mut self) -> Result<u16> { 
            Ok(0) 
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let data = vec![b'3', b'1', b'2', b'.'];
    let mut reader = MockReader { data, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };
    
    deserializer.ignore_integer().expect("Should not fail");
}

#[test]
fn test_ignore_integer_leading_zero() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { 
            Ok(Reference::new("")) 
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { 
            Ok(Reference::new(&[])) 
        }

        fn ignore_str(&mut self) -> Result<()> { 
            Ok(()) 
        }

        fn decode_hex_escape(&mut self) -> Result<u16> { 
            Ok(0) 
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let data = vec![b'0', b'1'];
    let mut reader = MockReader { data, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_with_invalid_characters() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { 
            Ok(Reference::new("")) 
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { 
            Ok(Reference::new(&[])) 
        }

        fn ignore_str(&mut self) -> Result<()> { 
            Ok(()) 
        }

        fn decode_hex_escape(&mut self) -> Result<u16> { 
            Ok(0) 
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let data = vec![b'2', b'#']; // '#' is invalid
    let mut reader = MockReader { data, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_with_exponent() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl Read<'static> for MockReader {
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
                let byte = self.data[self.position];
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0) // Simplified for testing
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { 
            Ok(Reference::new("")) 
        }
        
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { 
            Ok(Reference::new(&[])) 
        }

        fn ignore_str(&mut self) -> Result<()> { 
            Ok(()) 
        }

        fn decode_hex_escape(&mut self) -> Result<u16> { 
            Ok(0) 
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let data = vec![b'5', b'e', b'2'];
    let mut reader = MockReader { data, position: 0 };
    let mut deserializer = Deserializer { read: reader, scratch: Vec::new(), remaining_depth: 128 };

    deserializer.ignore_integer().expect("Should not fail");
}

