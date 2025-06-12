// Answer 0

#[test]
fn test_parse_unicode_escape_low_value() {
    use std::io::Cursor;
    use std::io::Read;

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, &'static str> {
            // Simulating a valid low hex escape sequence (0x0061 for 'a')
            if self.position < self.data.len() {
                let val = self.data[self.position];
                self.position += 1;
                return Ok(val as u32);
            }
            Err("EOF")
        }

        fn discard(&mut self) {
            // No-op for mock
        }

        fn peek_or_eof(&mut self) -> Result<u8, &'static str> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err("EOF")
            }
        }
    }

    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let available = self.data.len() - self.position;
            let to_read = buf.len().min(available);
            buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
            self.position += to_read;
            Ok(to_read)
        }
    }

    let mut read = MockRead::new(vec![0x61]); // Setting up input for 'a'
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0x61]); // Check that the push_wtf8_codepoint worked correctly
}

#[test]
fn test_parse_unicode_escape_with_surrogate() {
    use std::io::Cursor;
    use std::io::Read;

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, &'static str> {
            // Simulating a valid surrogate pair (0xD800 for leading surrogate)
            if self.position < self.data.len() {
                let val = self.data[self.position];
                self.position += 1;
                return Ok(val as u32);
            }
            Err("EOF")
        }

        fn discard(&mut self) {
            // No-op for mock
        }

        fn peek_or_eof(&mut self) -> Result<u8, &'static str> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Err("EOF")
            }
        }
    }

    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let available = self.data.len() - self.position;
            let to_read = buf.len().min(available);
            buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
            self.position += to_read;
            Ok(to_read)
        }
    }

    let mut read = MockRead::new(vec![0xD8, 0x00]); // Leading surrogate
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    
    assert!(result.is_ok());
    // Check that the leading surrogate was handled correctly
}

