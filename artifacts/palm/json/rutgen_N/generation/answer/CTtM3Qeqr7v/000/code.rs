// Answer 0

#[derive(Debug)]
struct MyStruct {
    index: usize,
}

impl MyStruct {
    fn byte_offset(&self) -> usize {
        self.index
    }
}

#[test]
fn test_byte_offset_zero() {
    let my_struct = MyStruct { index: 0 };
    assert_eq!(my_struct.byte_offset(), 0);
}

#[test]
fn test_byte_offset_positive() {
    let my_struct = MyStruct { index: 42 };
    assert_eq!(my_struct.byte_offset(), 42);
}

#[test]
fn test_byte_offset_large_value() {
    let my_struct = MyStruct { index: usize::MAX };
    assert_eq!(my_struct.byte_offset(), usize::MAX);
}

