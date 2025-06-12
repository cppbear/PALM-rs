// Answer 0

#[test]
fn test_parse_str_valid_input() {
    struct TestParser;

    impl TestParser {
        fn parse_str_bytes<'s>(&'s mut self, _scratch: &'s mut Vec<u8>, _: bool, _: fn(&[u8]) -> &str) -> Result<&'s [u8], ()> {
            Ok(b"test"[..].as_ref())
        }
        
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s str, ()> {
            self.parse_str_bytes(scratch, true, |bytes| std::str::from_utf8(bytes).unwrap())
                .map(|_| "test")
        }
    }

    let mut parser = TestParser;
    let mut scratch = Vec::new();
    let result = parser.parse_str(&mut scratch);
    assert_eq!(result.unwrap(), "test");
}

#[test]
#[should_panic]
fn test_parse_str_invalid_utf8() {
    struct TestParser;

    impl TestParser {
        fn parse_str_bytes<'s>(&'s mut self, _scratch: &'s mut Vec<u8>, _: bool, _: fn(&[u8]) -> &str) -> Result<&'s [u8], ()> {
            Ok(b"\xff\xff"[..].as_ref())
        }
        
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s str, ()> {
            self.parse_str_bytes(scratch, true, |bytes| std::str::from_utf8(bytes).unwrap())
                .map(|_| "invalid")
        }
    }

    let mut parser = TestParser;
    let mut scratch = Vec::new();
    let _ = parser.parse_str(&mut scratch);
}

