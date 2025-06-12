// Answer 0

#[test]
fn test_value_returns_correct_value() {
    #[derive(Debug, Copy, Clone)]
    struct TestKey(usize);
    
    #[derive(Debug, Copy, Clone)]
    struct TestValue(i32);
    
    let bucket = Bucket {
        hash: HashValue(1),
        key: TestKey(42),
        value: TestValue(24),
    };
    
    assert_eq!(bucket.value(), TestValue(24));
}

#[test]
#[should_panic]
fn test_value_panic_on_move() {
    #[derive(Debug, Copy, Clone)]
    struct TestKey(usize);
    
    #[derive(Debug, Copy, Clone)]
    struct TestValue(i32);
    
    let bucket = Bucket {
        hash: HashValue(1),
        key: TestKey(42),
        value: TestValue(24),
    };
    
    let _value = bucket.value(); // value will be moved here
    
    // Attempt to call value again should panic if we try to access it again
    let _ = bucket.value(); // This should cause a compilation error, hence kept the test
}

#[test]
fn test_value_multiple_calls() {
    #[derive(Debug, Copy, Clone)]
    struct TestKey(usize);
    
    #[derive(Debug, Copy, Clone)]
    struct TestValue(i32);
    
    let mut bucket = Bucket {
        hash: HashValue(1),
        key: TestKey(42),
        value: TestValue(24),
    };
    
    let value1 = bucket.value(); // First access
    assert_eq!(value1, TestValue(24));
    
    let value2 = bucket.value(); // Second access
    assert_eq!(value2, TestValue(24));
}

#[test]
fn test_value_with_different_data() {
    #[derive(Debug, Copy, Clone)]
    struct TestKey(usize);
    
    #[derive(Debug, Copy, Clone)]
    struct TestValue(i32);
    
    let bucket = Bucket {
        hash: HashValue(2),
        key: TestKey(84),
        value: TestValue(99),
    };
    
    assert_eq!(bucket.value(), TestValue(99));
}

