// Answer 0

#[derive(Debug)]
struct Delegate {
    offset: usize,
}

impl Delegate {
    fn new(offset: usize) -> Self {
        Delegate { offset }
    }
    
    fn byte_offset(&self) -> usize {
        self.offset
    }
}

struct TestStruct {
    delegate: Delegate,
}

impl TestStruct {
    fn new(offset: usize) -> Self {
        TestStruct { delegate: Delegate::new(offset) }
    }

    fn byte_offset(&self) -> usize {
        self.delegate.byte_offset()
    }
}

#[test]
fn test_byte_offset_zero() {
    let test_struct = TestStruct::new(0);
    assert_eq!(test_struct.byte_offset(), 0);
}

#[test]
fn test_byte_offset_positive() {
    let test_struct = TestStruct::new(10);
    assert_eq!(test_struct.byte_offset(), 10);
}

#[test]
fn test_byte_offset_large() {
    let test_struct = TestStruct::new(1_000_000);
    assert_eq!(test_struct.byte_offset(), 1_000_000);
}

#[test]
fn test_byte_offset_boundary() {
    let test_struct = TestStruct::new(usize::MAX);
    assert_eq!(test_struct.byte_offset(), usize::MAX);
}

