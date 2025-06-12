// Answer 0

#[test]
fn test_parse_str_bytes_no_escape() {
    struct TestRead {
        slice: &'static [u8],
        index: usize,
    }

    impl TestRead {
        fn new(slice: &'static [u8]) -> Self {
            TestRead { slice, index: 0 }
        }

        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestRead::new(b"simple string");
    let result = reader.parse_str_bytes(&mut scratch, true, |_, data| {
        assert_eq!(data, b"simple string");
        Ok(data)
    });

    assert!(result.is_ok());
}

#[test]
fn test_parse_str_bytes_escape_sequence() {
    struct TestRead {
        slice: &'static [u8],
        index: usize,
    }

    impl TestRead {
        fn new(slice: &'static [u8]) -> Self {
            TestRead { slice, index: 0 }
        }

        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    fn parse_escape(_reader: &mut TestRead, _validate: bool, _scratch: &mut Vec<u8>) -> Result<(), ()> {
        Ok(())
    }

    let mut scratch = Vec::new();
    let mut reader = TestRead::new(b"string with escape \\n");
    let result = reader.parse_str_bytes(&mut scratch, true, |_, data| {
        assert_eq!(data, b"string with escape ");
        Ok(data)
    });

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_parse_str_bytes_control_character() {
    struct TestRead {
        slice: &'static [u8],
        index: usize,
    }

    impl TestRead {
        fn new(slice: &'static [u8]) -> Self {
            TestRead { slice, index: 0 }
        }

        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestRead::new(b"control char: \x01");
    let _ = reader.parse_str_bytes(&mut scratch, true, |_, _| Ok(&b""[..]));
}

#[test]
fn test_parse_str_bytes_escaped_character() {
    struct TestRead {
        slice: &'static [u8],
        index: usize,
    }

    impl TestRead {
        fn new(slice: &'static [u8]) -> Self {
            TestRead { slice, index: 0 }
        }

        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    fn parse_escape(_reader: &mut TestRead, _validate: bool, _scratch: &mut Vec<u8>) -> Result<(), ()> {
        Ok(())
    }

    let mut scratch = Vec::new();
    let mut reader = TestRead::new(b"escaped \\n characters");
    let start_index = reader.index;
    reader.skip_to_escape(true);
    let end_index = reader.index;

    let result = reader.parse_str_bytes(&mut scratch, true, |_, data| {
        assert_eq!(data, &reader.slice[start_index..end_index]);
        Ok(data)
    });

    assert!(result.is_ok());
}

