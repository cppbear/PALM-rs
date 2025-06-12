// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestParser {
        input: Vec<u8>,
        current: usize,
    }

    impl TestParser {
        fn new(input: &[u8]) -> Self {
            Self {
                input: input.to_vec(),
                current: 0,
            }
        }

        fn parse_whitespace(&mut self) -> Option<u8> {
            while self.current < self.input.len() {
                let byte = self.input[self.current];
                if byte.is_ascii_whitespace() {
                    self.current += 1;
                } else {
                    return Some(byte);
                }
            }
            None
        }

        fn eat_char(&mut self) {
            self.current += 1;
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error { code }
        }
    }

    impl Parser for TestParser {
        fn end_map(&mut self) -> Result<()> {
            match self.parse_whitespace() {
                Some(b'}') => {
                    self.eat_char();
                    Ok(())
                }
                Some(b',') => Err(self.peek_error(ErrorCode::TrailingComma)),
                Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
                None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
            }
        }
    }

    #[test]
    fn test_end_map_valid_closing_brace() {
        let mut parser = TestParser::new(b" }");
        assert!(parser.end_map().is_ok());
    }

    #[test]
    fn test_end_map_trailing_comma() {
        let mut parser = TestParser::new(b" ,");
        let result = parser.end_map();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::TrailingComma);
    }

    #[test]
    fn test_end_map_trailing_characters() {
        let mut parser = TestParser::new(b" abc");
        let result = parser.end_map();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::TrailingCharacters);
    }

    #[test]
    fn test_end_map_eof() {
        let mut parser = TestParser::new(b"");
        let result = parser.end_map();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::EofWhileParsingObject);
    }
}

