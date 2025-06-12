// Answer 0

#[derive(Default)]
struct Writer {
    bytes: Vec<u8>,
    offset: usize,
}

impl Writer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.offset + s.len() > self.bytes.len() {
            Err(std::fmt::Error)
        } else {
            self.bytes[self.offset..self.offset + s.len()].copy_from_slice(s.as_bytes());
            self.offset += s.len();
            Ok(())
        }
    }
}

#[test]
fn test_write_str_exceeds_capacity() {
    let mut writer = Writer {
        bytes: vec![0; 5], // create a writer with a byte buffer of size 5
        offset: 4, // set offset near the end of buffer
    };
    
    let result = writer.write_str("test"); // length is 4, offset is 4, total would be 8 which exceeds capacity
    assert!(result.is_err()); // Expecting an error
}

#[test]
fn test_write_str_exact_capacity() {
    let mut writer = Writer {
        bytes: vec![0; 5], // create a writer with a byte buffer of size 5
        offset: 0, // starting from the beginning
    };
    
    let result = writer.write_str("hello"); // length is 5, should fit exactly into the buffer
    assert!(result.is_ok()); // Expecting success
    assert_eq!(&writer.bytes[0..5], b"hello"); // Verify that the data is written correctly
} 

#[test]
fn test_write_str_over_capacity() {
    let mut writer = Writer {
        bytes: vec![0; 10], // create a writer with a byte buffer of size 10
        offset: 8, // set offset such that adding 3 will exceed
    };
    
    let result = writer.write_str("123"); // length is 3, offset is 8, total would be 11 which exceeds capacity
    assert!(result.is_err()); // Expecting an error
}

