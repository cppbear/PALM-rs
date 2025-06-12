// Answer 0

#[derive(Debug)]
struct MockCClass {
    byte: Option<u8>,
}

impl MockCClass {
    fn c_class_bytes(&mut self, bytes: &[hir::ClassBytesRange]) -> Result {
        if bytes.is_empty() {
            Err("No bytes provided")
        } else {
            self.byte = Some(bytes[0].start());
            Ok(())
        }
    }
}

#[test]
fn test_c_byte_success() {
    let mut mock = MockCClass { byte: None };
    let result = mock.c_byte(5);
    assert!(result.is_ok());
    assert_eq!(mock.byte, Some(5));
}

#[test]
fn test_c_byte_empty() {
    let mut mock = MockCClass { byte: None };
    let result = mock.c_class_bytes(&[]);
    assert!(result.is_err());
}

