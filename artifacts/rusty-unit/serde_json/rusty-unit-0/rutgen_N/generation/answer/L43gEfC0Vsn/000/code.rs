// Answer 0

#[test]
fn test_parse_str_raw() {
    struct TestStruct;

    impl TestStruct {
        fn parse_str_bytes<'s>(&'s mut self, _: &'s mut Vec<u8>, _: bool, f: fn(&(), &[u8]) -> Result<&[u8], ()>) -> Result<&[u8], ()> {
            let bytes = b"test string";
            f(&(), bytes)
        }
        
        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s [u8], ()> {
            self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
        }
    }

    let mut test_struct = TestStruct;
    let mut scratch = Vec::new();
    
    let result = test_struct.parse_str_raw(&mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"test string");
}

#[test]
fn test_parse_str_raw_empty_scratch() {
    struct TestStruct;

    impl TestStruct {
        fn parse_str_bytes<'s>(&'s mut self, _: &'s mut Vec<u8>, _: bool, f: fn(&(), &[u8]) -> Result<&[u8], ()>) -> Result<&[u8], ()> {
            let bytes = b"empty test";
            f(&(), bytes)
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s [u8], ()> {
            self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
        }
    }

    let mut test_struct = TestStruct;
    let mut scratch = Vec::new();
    
    let result = test_struct.parse_str_raw(&mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"empty test");
}

