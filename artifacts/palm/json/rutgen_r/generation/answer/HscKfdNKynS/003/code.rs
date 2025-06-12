// Answer 0

#[test]
fn test_parse_str_bytes_valid() {
    struct MockParser {
        bytes: Vec<u8>,
        index: usize,
    }
    
    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.bytes.len() {
                let ch = self.bytes[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err("EOF".into())
            }
        }

        fn parse_escape(&mut self, validate: bool, scratch: &mut Vec<u8>) -> Result<()> {
            if validate {
                return Err("Invalid escape".into());
            }
            // Simulate parsing an escape character
            scratch.push(b'\\');
            Ok(())
        }
    }

    let mut parser = MockParser { bytes: vec![b'"', b'h', b'e', b'l', b'l', b'o', b'"'], index: 0 };
    let mut scratch: Vec<u8> = Vec::new();
    let result: Result<Vec<u8>> = parse_str_bytes(&mut parser, &mut scratch, false, |_, vec| Ok(vec.to_vec()));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"hello".to_vec());
}

#[test]
#[should_panic(expected = "Invalid escape")]
fn test_parse_str_bytes_invalid_escape_with_validation() {
    struct MockParser {
        bytes: Vec<u8>,
        index: usize,
    }
    
    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.bytes.len() {
                let ch = self.bytes[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err("EOF".into())
            }
        }

        fn parse_escape(&mut self, validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            if validate {
                return Err("Invalid escape".into());
            }
            Ok(())
        }
    }

    let mut parser = MockParser { bytes: vec![b'"', b'\\', b'h', b'e', b'l', b'l', b'o', b'"'], index: 0 };
    let mut scratch: Vec<u8> = Vec::new();
    
    let _: Result<Vec<u8>> = parse_str_bytes(&mut parser, &mut scratch, true, |_, _| Ok(vec![]));
}

#[test]
fn test_parse_str_bytes_control_character_error() {
    struct MockParser {
        bytes: Vec<u8>,
        index: usize,
    }
    
    impl MockParser {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.bytes.len() {
                let ch = self.bytes[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err("EOF".into())
            }
        }

        fn parse_escape(&mut self, validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            if validate {
                return Err("Invalid escape".into());
            }
            Ok(())
        }
    }

    let mut parser = MockParser { bytes: vec![b'"', b'\x00', b'h', b'e', b'l', b'l', b'o', b'"'], index: 0 };
    let mut scratch: Vec<u8> = Vec::new();
    
    let result = parse_str_bytes(&mut parser, &mut scratch, true, |_, _| Ok(vec![]));
    
    assert!(result.is_err());
}

