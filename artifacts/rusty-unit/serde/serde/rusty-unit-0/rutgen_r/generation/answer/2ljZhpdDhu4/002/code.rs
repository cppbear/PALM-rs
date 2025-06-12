// Answer 0

#[test]
fn test_write_str_boundaries_exact_fit() {
    struct TestWriter {
        bytes: Vec<u8>,
        offset: usize,
    }
    
    impl TestWriter {
        fn new(size: usize) -> Self {
            TestWriter {
                bytes: vec![0u8; size],
                offset: 0,
            }
        }
        
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

    let mut writer = TestWriter::new(5);
    let result = writer.write_str("hello");
    assert!(result.is_ok());
    assert_eq!(writer.bytes, b"hello");
}

#[test]
fn test_write_str_boundary_exact_fit_empty_string() {
    struct TestWriter {
        bytes: Vec<u8>,
        offset: usize,
    }
    
    impl TestWriter {
        fn new(size: usize) -> Self {
            TestWriter {
                bytes: vec![0u8; size],
                offset: 0,
            }
        }
        
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

    let mut writer = TestWriter::new(5);
    let result = writer.write_str("");
    assert!(result.is_ok());
    assert_eq!(writer.bytes, b"");
}

