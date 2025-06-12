// Answer 0

fn write_encoded_bytes(encoded: &[u8]) -> Result<(), &'static str> {
    // Simulate the function behavior based on the description
    if std::str::from_utf8(encoded).is_err() {
        return Err("base64 data was not utf8");
    }
    Ok(())
}

struct MockWriter {
    was_written: bool,
}

impl MockWriter {
    fn write_str(&mut self, _s: &str) {
        self.was_written = true;
    }
}

#[test]
fn test_write_encoded_bytes_valid_utf8() {
    let mut writer = MockWriter { was_written: false };
    let encoded: &[u8] = b"valid_utf8";
    
    let result = write_encoded_bytes(encoded);
    
    assert!(result.is_ok());
    assert!(writer.was_written);
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut writer = MockWriter { was_written: false };
    let encoded: &[u8] = &[0xFF]; // Invalid UTF-8 byte
    
    let _ = write_encoded_bytes(encoded).expect("Expected to panic");
}

#[test]
fn test_write_encoded_bytes_empty() {
    let mut writer = MockWriter { was_written: false };
    let encoded: &[u8] = b"";
    
    let result = write_encoded_bytes(encoded);
    
    assert!(result.is_ok());
    assert!(!writer.was_written);
}

#[test]
fn test_write_encoded_bytes_partial_utf8() {
    let mut writer = MockWriter { was_written: false };
    let encoded: &[u8] = b"partial_\xFF_utf8"; // Contains an invalid UTF-8 byte
    
    let result = write_encoded_bytes(encoded);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "base64 data was not utf8");
}

