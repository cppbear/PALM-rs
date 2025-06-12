// Answer 0

#[derive(Default)]
struct ChunkedEncoder {
    string: String,
}

impl ChunkedEncoder {
    fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), String> {
        self.string.push_str(std::str::from_utf8(s).unwrap());
        Ok(())
    }
}

#[test]
fn test_write_encoded_bytes_with_valid_utf8() {
    let mut encoder = ChunkedEncoder::default();
    let input = b"Hello, world!";
    let result = encoder.write_encoded_bytes(input);
    assert!(result.is_ok());
    assert_eq!(encoder.string, "Hello, world!");
}

#[test]
#[should_panic]
fn test_write_encoded_bytes_with_invalid_utf8() {
    let mut encoder = ChunkedEncoder::default();
    let input = &[0, 159, 146, 150]; // Invalid UTF-8
    let _ = encoder.write_encoded_bytes(input);
}

