// Answer 0


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Read};

    struct TestReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(buffer: Vec<u8>) -> Self {
            Self { buffer, position: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16, io::Error> {
            // Simulating valid hexadecimal escapes here
            if self.position < self.buffer.len() {
                let hex = self.buffer[self.position];
                self.position += 1;
                return Ok(hex);
            }
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
        }

        fn discard(&mut self) {
            // Simulate discarding a character
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }

        fn peek_or_eof(&self) -> Result<u8, io::Error> {
            if self.position < self.buffer.len() {
                Ok(self.buffer[self.position])
            } else {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
            }
        }
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
            let n = std::cmp::min(buf.len(), self.buffer.len() - self.position);
            buf[..n].copy_from_slice(&self.buffer[self.position..self.position + n]);
            self.position += n;
            Ok(n)
        }
    }

    #[test]
    #[should_panic]
    fn test_parse_unicode_escape_invalid_lone_leading_surrogate() {
        let mut scratch = Vec::new();
        let mut reader = TestReader::new(vec![0xD8, 0x00]); // Starting with leading surrogate 0xD800

        let result = parse_unicode_escape(&mut reader, true, &mut scratch);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_parse_unicode_escape_invalid_lone_leading_surrogate2() {
        let mut scratch = Vec::new();
        let mut reader = TestReader::new(vec![0xDC, 0x00]); // Leading surrogate 0xDC00 is in range

        let result = parse_unicode_escape(&mut reader, true, &mut scratch);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unicode_escape_no_surrogate() {
        let mut scratch = Vec::new();
        let mut reader = TestReader::new(vec![0x61]); // 'a' in UTF-8

        let result = parse_unicode_escape(&mut reader, false, &mut scratch);
        assert!(result.is_ok());
        assert_eq!(scratch, vec![0x61]); // Check if 'a' was correctly added to scratch
    }

    #[test]
    #[should_panic]
    fn test_parse_unicode_escape_eof_after_leading() {
        let mut scratch = Vec::new();
        let mut reader = TestReader::new(vec![0xD8]); // Just the leading surrogate 0xD800

        let result = parse_unicode_escape(&mut reader, true, &mut scratch);
        assert!(result.is_err());
    }
}


