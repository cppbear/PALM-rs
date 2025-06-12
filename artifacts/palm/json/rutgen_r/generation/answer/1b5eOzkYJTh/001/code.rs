// Answer 0

#[test]
fn test_parse_str_with_valid_data() {
    struct Reader {
        data: &'static str,
    }
    
    impl Reader {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s str, ()> {
            scratch.extend_from_slice(self.data.as_bytes());
            Ok(self.data)
        }
    }

    let mut scratch = Vec::new();
    let mut reader = Reader { data: "test string" };

    let result = reader.parse_str(&mut scratch);
    assert_eq!(result.unwrap(), "test string");
    assert_eq!(scratch, b"test string");
}

#[test]
#[should_panic]
fn test_parse_str_with_empty_data() {
    struct Reader {
        data: &'static str,
    }
    
    impl Reader {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s str, ()> {
            if self.data.is_empty() {
                panic!("data cannot be empty");
            }
            scratch.extend_from_slice(self.data.as_bytes());
            Ok(self.data)
        }
    }

    let mut scratch = Vec::new();
    let mut reader = Reader { data: "" };

    reader.parse_str(&mut scratch);
}

#[test]
fn test_parse_str_with_special_characters() {
    struct Reader {
        data: &'static str,
    }
    
    impl Reader {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s str, ()> {
            scratch.extend_from_slice(self.data.as_bytes());
            Ok(self.data)
        }
    }

    let mut scratch = Vec::new();
    let mut reader = Reader { data: "special chars: ~!@#$%^&*()" };

    let result = reader.parse_str(&mut scratch);
    assert_eq!(result.unwrap(), "special chars: ~!@#$%^&*()");
    assert_eq!(scratch, b"special chars: ~!@#$%^&*()");
}

#[test]
fn test_parse_str_with_large_data() {
    struct Reader {
        data: String,
    }
    
    impl Reader {
        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<&'s str, ()> {
            scratch.extend_from_slice(self.data.as_bytes());
            Ok(&self.data)
        }
    }

    let mut scratch = Vec::new();
    let data = "a".repeat(1_000_000); // large data
    let mut reader = Reader { data };

    let result = reader.parse_str(&mut scratch);
    assert_eq!(result.unwrap(), &*reader.data);
    assert_eq!(scratch.len(), 1_000_000);
}

