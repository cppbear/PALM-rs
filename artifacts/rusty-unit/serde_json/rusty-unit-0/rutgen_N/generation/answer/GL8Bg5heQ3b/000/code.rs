// Answer 0

#[test]
fn test_line() {
    struct TestStruct {
        line: usize,
    }
    
    impl TestStruct {
        pub fn new(line: usize) -> Self {
            TestStruct { line }
        }
    }
    
    let instance = TestStruct::new(42);
    assert_eq!(instance.line(), 42);
}

#[test]
fn test_line_zero() {
    struct TestStruct {
        line: usize,
    }
    
    impl TestStruct {
        pub fn new(line: usize) -> Self {
            TestStruct { line }
        }
    }
    
    let instance = TestStruct::new(0);
    assert_eq!(instance.line(), 0);
}

#[test]
fn test_line_large_number() {
    struct TestStruct {
        line: usize,
    }
    
    impl TestStruct {
        pub fn new(line: usize) -> Self {
            TestStruct { line }
        }
    }
    
    let instance = TestStruct::new(usize::MAX);
    assert_eq!(instance.line(), usize::MAX);
}

