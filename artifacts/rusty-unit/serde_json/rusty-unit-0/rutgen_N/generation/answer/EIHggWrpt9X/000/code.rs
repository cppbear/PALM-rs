// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::de::Deserializer;
    
    struct TestDeserializer {
        input: Vec<u8>,
        position: usize,
    }

    impl TestDeserializer {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn parse_whitespace(&mut self) -> Result<Option<()>> {
            while self.position < self.input.len() && self.input[self.position].is_ascii_whitespace() {
                self.position += 1;
            }
            if self.position == self.input.len() {
                Ok(None)
            } else {
                Ok(Some(()))
            }
        }

        fn peek_error(&self, _error_code: ErrorCode) -> serde_json::Result<()> {
            Err(serde_json::Error::custom("Trailing characters"))
        }
    }

    impl Deserializer for TestDeserializer {
        fn end(&mut self) -> Result<()> {
            match self.parse_whitespace()? {
                Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
                None => Ok(()),
            }
        }
    }

    #[test]
    fn test_end_with_valid_input() {
        let mut deserializer = TestDeserializer::new(b"   ".to_vec());
        let result = deserializer.end();
        assert!(result.is_ok());
    }

    #[test]
    fn test_end_with_trailing_characters() {
        let mut deserializer = TestDeserializer::new(b"valid data   extra".to_vec());
        let result = deserializer.end();
        assert!(result.is_err());
    }

    #[test]
    fn test_end_with_empty_input() {
        let mut deserializer = TestDeserializer::new(b"".to_vec());
        let result = deserializer.end();
        assert!(result.is_ok());
    }
}

