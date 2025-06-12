// Answer 0

#[derive(Debug)]
struct TestStruct {
    index: usize,
}

impl TestStruct {
    fn byte_offset(&self) -> usize {
        self.index
    }
}

#[test]
fn test_byte_offset_zero() {
    let test_instance = TestStruct { index: 0 };
    assert_eq!(test_instance.byte_offset(), 0);
}

#[test]
fn test_byte_offset_positive() {
    let test_instance = TestStruct { index: 42 };
    assert_eq!(test_instance.byte_offset(), 42);
}

#[test]
fn test_byte_offset_large() {
    let test_instance = TestStruct { index: usize::MAX };
    assert_eq!(test_instance.byte_offset(), usize::MAX);
}

#[test]
fn test_byte_offset_boundary() {
    let test_instance = TestStruct { index: 1 };
    assert_eq!(test_instance.byte_offset(), 1);
}

