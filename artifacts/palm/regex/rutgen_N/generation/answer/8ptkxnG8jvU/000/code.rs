// Answer 0

#[derive(Debug)]
struct TestStruct {
    limit_class: usize,
}

impl TestStruct {
    pub fn new(limit_class: usize) -> Self {
        TestStruct { limit_class }
    }
    
    pub fn limit_class(&self) -> usize {
        self.limit_class
    }
}

#[test]
fn test_limit_class_zero() {
    let test_instance = TestStruct::new(0);
    assert_eq!(test_instance.limit_class(), 0);
}

#[test]
fn test_limit_class_positive() {
    let test_instance = TestStruct::new(5);
    assert_eq!(test_instance.limit_class(), 5);
}

#[test]
fn test_limit_class_large() {
    let test_instance = TestStruct::new(usize::MAX);
    assert_eq!(test_instance.limit_class(), usize::MAX);
}

