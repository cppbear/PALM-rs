// Answer 0

#[derive(Debug)]
struct TestStruct {
    limit_size: usize,
}

impl TestStruct {
    pub fn new(limit_size: usize) -> Self {
        TestStruct { limit_size }
    }
    
    pub fn limit_size(&self) -> usize {
        self.limit_size
    }
}

#[test]
fn test_limit_size_zero() {
    let test_instance = TestStruct::new(0);
    assert_eq!(test_instance.limit_size(), 0);
}

#[test]
fn test_limit_size_small_value() {
    let test_instance = TestStruct::new(10);
    assert_eq!(test_instance.limit_size(), 10);
}

#[test]
fn test_limit_size_large_value() {
    let test_instance = TestStruct::new(1_000_000);
    assert_eq!(test_instance.limit_size(), 1_000_000);
}

#[test]
fn test_limit_size_boundary_value() {
    let test_instance = TestStruct::new(usize::MAX);
    assert_eq!(test_instance.limit_size(), usize::MAX);
}

