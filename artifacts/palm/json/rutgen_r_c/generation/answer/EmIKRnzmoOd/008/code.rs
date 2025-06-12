// Answer 0

fn test_scan_integer128_valid_leading_zero() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: &[u8]) -> Self {
            Self { data: data.to_vec(), position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let c = self.data[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Ok(0) // return null byte if at end
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
    }

    let mut reader = TestReader::new(b"0");
    let mut buf = String::new();
    match reader.scan_integer128(&mut buf) {
        Ok(()) => assert_eq!(buf, "0"),
        Err(_) => panic!("Expected Ok result"),
    }
    Ok(())
}

fn test_scan_integer128_invalid_leading_zero_followed_by_digit() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: &[u8]) -> Self {
            Self { data: data.to_vec(), position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let c = self.data[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Ok(0) // return null byte if at end
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
    }

    let mut reader = TestReader::new(b"00");
    let mut buf = String::new();
    match reader.scan_integer128(&mut buf) {
        Ok(_) => panic!("Expected Err due to leading zeros"),
        Err(err) => assert!(matches!(err, ErrorCode::InvalidNumber)), // ensure the error is as expected
    }
    Ok(())
}

fn test_scan_integer128_invalid_character() -> Result<()> {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: &[u8]) -> Self {
            Self { data: data.to_vec(), position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let c = self.data[self.position];
                self.position += 1;
                Ok(Some(c))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&mut self) -> Result<u8> {
            if self.position < self.data.len() {
                Ok(self.data[self.position])
            } else {
                Ok(0) // return null byte if at end
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
    }

    let mut reader = TestReader::new(b"a");
    let mut buf = String::new();
    match reader.scan_integer128(&mut buf) {
        Ok(_) => panic!("Expected Err due to invalid character"),
        Err(err) => assert!(matches!(err, ErrorCode::InvalidNumber)), // ensure the error is as expected
    }
    Ok(())
}

fn main() {
    test_scan_integer128_valid_leading_zero().unwrap();
    test_scan_integer128_invalid_leading_zero_followed_by_digit().unwrap();
    test_scan_integer128_invalid_character().unwrap();
}

