// Answer 0

#[test]
fn test_parse_str_success() {
    struct MockParser {
        data: Vec<u8>,
    }
    
    impl MockParser {
        fn parse_str_bytes<'s>(&'s mut self, _scratch: &'s mut Vec<u8>, _flag: bool, _callback: fn(&[u8]) -> &'s str) -> Result<&'s str, &'static str> {
            if self.data.is_empty() {
                Err("No data to parse")
            } else {
                Ok(callback(&self.data))
            }
        }
    }
    
    fn as_str(bytes: &[u8]) -> &str {
        std::str::from_utf8(bytes).unwrap()
    }
    
    let mut parser = MockParser { data: b"hello".to_vec() };
    let mut scratch = Vec::new();
    let result: Result<&str, &str> = parser.parse_str(&mut scratch);
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_parse_str_empty_data() {
    struct MockParser {
        data: Vec<u8>,
    }
    
    impl MockParser {
        fn parse_str_bytes<'s>(&'s mut self, _scratch: &'s mut Vec<u8>, _flag: bool, _callback: fn(&[u8]) -> &'s str) -> Result<&'s str, &'static str> {
            if self.data.is_empty() {
                Err("No data to parse")
            } else {
                Ok(callback(&self.data))
            }
        }
    }
    
    fn as_str(bytes: &[u8]) -> &str {
        std::str::from_utf8(bytes).unwrap()
    }
    
    let mut parser = MockParser { data: Vec::new() };
    let mut scratch = Vec::new();
    let result: Result<&str, &str> = parser.parse_str(&mut scratch);
    assert_eq!(result.unwrap_err(), "No data to parse");
}

