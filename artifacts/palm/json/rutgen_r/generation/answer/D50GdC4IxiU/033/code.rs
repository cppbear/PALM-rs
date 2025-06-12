// Answer 0

#[test]
fn test_parse_unicode_escape_lone_leading_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, ()> {
            if self.position < self.data.len() {
                let val = self.data[self.position];
                self.position += 1;
                Ok(val as u16)
            } else {
                Err(())
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&self) -> Result<u8, ()> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, b'\\', b'u']); // Setting up to assure that leading surrogate condition triggers.
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    
    assert!(result.is_err());
}

#[test]
fn test_parse_unicode_escape_unexpected_end_of_hex_escape() {
    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, ()> {
            if self.position < self.data.len() {
                let val = self.data[self.position];
                self.position += 1;
                Ok(val as u16)
            } else {
                Err(())
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }

        fn peek_or_eof(&self) -> Result<u8, ()> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err(())
            }
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, b'\\', b'x']); // Here, we provide an unexpected end character.
    let result = parse_unicode_escape(&mut read, true, &mut scratch);
    
    assert!(result.is_err());
}

