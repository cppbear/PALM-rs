// Answer 0

#[test]
fn test_is_empty_with_zero_length() {
    struct TestStruct {
        len: usize,
    }
    
    let test_instance = TestStruct { len: 0 };
    assert!(test_instance.is_empty());
}

#[test]
fn test_is_empty_with_non_zero_length() {
    struct TestStruct {
        len: usize,
    }
    
    let test_instance = TestStruct { len: 5 };
    assert!(!test_instance.is_empty());
}

#[test]
fn test_is_empty_with_another_non_zero_length() {
    struct TestStruct {
        len: usize,
    }
    
    let test_instance = TestStruct { len: 1 };
    assert!(!test_instance.is_empty());
}

