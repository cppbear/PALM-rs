// Answer 0

#[test]
fn test_parse_str_bytes_empty_scratch_fast_path() {
    struct TestReader {
        index: usize,
        slice: &'static [u8],
    }

    impl TestReader {
        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    fn result_fn<'s>(_: &mut TestReader, borrowed: &'s [u8]) -> Result<&'s str, &'static str> {
        std::str::from_utf8(borrowed).map_err(|_| "Invalid UTF-8")
    }
    
    let mut reader = TestReader { index: 0, slice: b"hello\"world" };
    let mut scratch = Vec::new();
    let validate = true;

    let result = parse_str_bytes(&mut reader, &mut scratch, validate, result_fn);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_ref(), "hello");
    assert_eq!(reader.index, 9);
}

#[test]
fn test_parse_str_bytes_non_empty_scratch_with_escape() {
    struct TestReader {
        index: usize,
        slice: &'static [u8],
    }

    impl TestReader {
        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    fn result_fn<'s>(_: &mut TestReader, scratch: &'s [u8]) -> Result<&'s str, &'static str> {
        std::str::from_utf8(scratch).map_err(|_| "Invalid UTF-8")
    }
    
    let mut reader = TestReader { index: 0, slice: b"hello\\\"world" };
    let mut scratch = Vec::new();
    let validate = true;

    let result = parse_str_bytes(&mut reader, &mut scratch, validate, result_fn);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_ref(), "hello\\\"world"); // Check for escaping
    assert_eq!(reader.index, 11);
}

#[test]
fn test_parse_str_bytes_control_character() {
    struct TestReader {
        index: usize,
        slice: &'static [u8],
    }

    impl TestReader {
        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    fn result_fn<'s>(_: &mut TestReader, _: &'s [u8]) -> Result<&'s str, &'static str> {
        Err("Should not reach here due to control character")
    }
    
    let mut reader = TestReader { index: 0, slice: b"hello\x00world" }; // Control character
    let mut scratch = Vec::new();
    let validate = true;

    let result = parse_str_bytes(&mut reader, &mut scratch, validate, result_fn);
    
    assert!(result.is_err());
}

