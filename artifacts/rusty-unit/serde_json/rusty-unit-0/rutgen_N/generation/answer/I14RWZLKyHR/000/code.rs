// Answer 0

#[derive(Debug)]
struct TestStruct {
    col: usize,
}

impl TestStruct {
    pub fn new(col: usize) -> Self {
        TestStruct { col }
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[test]
fn test_col() {
    let instance = TestStruct::new(5);
    assert_eq!(instance.col(), 5);
}

#[test]
fn test_col_zero() {
    let instance = TestStruct::new(0);
    assert_eq!(instance.col(), 0);
}

#[test]
fn test_col_large_number() {
    let instance = TestStruct::new(usize::MAX);
    assert_eq!(instance.col(), usize::MAX);
}

