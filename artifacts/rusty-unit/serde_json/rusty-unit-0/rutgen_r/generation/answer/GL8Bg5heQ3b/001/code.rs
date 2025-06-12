// Answer 0

#[derive(Debug)]
struct TestStruct {
    line: usize,
}

impl TestStruct {
    pub fn new(line: usize) -> Self {
        TestStruct { line }
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

#[test]
fn test_line_with_zero() {
    let test_struct = TestStruct::new(0);
    assert_eq!(test_struct.line(), 0);
}

#[test]
fn test_line_with_positive_value() {
    let test_struct = TestStruct::new(42);
    assert_eq!(test_struct.line(), 42);
}

#[test]
fn test_line_with_large_value() {
    let test_struct = TestStruct::new(1_000_000);
    assert_eq!(test_struct.line(), 1_000_000);
}

#[test]
fn test_line_with_max_usize() {
    let test_struct = TestStruct::new(usize::MAX);
    assert_eq!(test_struct.line(), usize::MAX);
}

