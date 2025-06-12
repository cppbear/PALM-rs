// Answer 0

#[test]
fn test_decode_hex_escape_valid_case() {
    struct TestReader {
        data: Vec<char>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                data: input.chars().collect(),
                pos: 0,
            }
        }

        fn next_or_eof(&mut self) -> Result<char> {
            if self.pos < self.data.len() {
                let result = Ok(self.data[self.pos]);
                self.pos += 1;
                result
            } else {
                Err(ErrorCode::UnexpectedEof)
            }
        }
    }

    let mut reader = TestReader::new("0041"); // Represents hex for 'A'
    let result = reader.decode_hex_escape();
    assert_eq!(result.unwrap(), 0x0041); // Expected value for 'A'
}

#[test]
#[should_panic]
fn test_decode_hex_escape_invalid_case() {
    struct TestReader {
        data: Vec<char>,
        pos: usize,
    }

    impl TestReader {
        fn new(input: &str) -> Self {
            Self {
                data: input.chars().collect(),
                pos: 0,
            }
        }

        fn next_or_eof(&mut self) -> Result<char> {
            if self.pos < self.data.len() {
                let result = Ok(self.data[self.pos]);
                self.pos += 1;
                result
            } else {
                Err(ErrorCode::UnexpectedEof)
            }
        }
    }

    impl TestReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next_or_eof().unwrap(); // Panic if EOF
            let b = self.next_or_eof().unwrap(); // Panic if EOF
            let c = self.next_or_eof().unwrap(); // Panic if EOF
            let d = self.next_or_eof().unwrap(); // Panic if EOF
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => panic!("Invalid escape"),
            }
        }
    }

    let mut reader = TestReader::new("XYZ9"); // Invalid hex digits
    let _ = reader.decode_hex_escape(); // Should panic due to invalid input
}

