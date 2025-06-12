// Answer 0

#[test]
fn write_encoded_bytes_valid_utf8() {
    struct ChunkedEncoder {
        string: String,
    }

    impl ChunkedEncoder {
        fn new() -> Self {
            ChunkedEncoder {
                string: String::new(),
            }
        }

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), std::str::Utf8Error> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = ChunkedEncoder::new();
    let input = b"Hello, World!";
    let result = encoder.write_encoded_bytes(input);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn write_encoded_bytes_invalid_utf8() {
    struct ChunkedEncoder {
        string: String,
    }

    impl ChunkedEncoder {
        fn new() -> Self {
            ChunkedEncoder {
                string: String::new(),
            }
        }

        fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), std::str::Utf8Error> {
            self.string.push_str(std::str::from_utf8(s).unwrap());
            Ok(())
        }
    }

    let mut encoder = ChunkedEncoder::new();
    let input = [0, 159, 146, 150]; // invalid UTF-8 bytes
    encoder.write_encoded_bytes(&input).unwrap(); // This should panic
}

