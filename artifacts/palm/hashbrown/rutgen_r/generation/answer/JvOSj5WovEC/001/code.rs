// Answer 0

#[derive(Debug)]
struct TestStruct {
    value: i32,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        TestStruct { value }
    }
}

// Testing the deref function
#[test]
fn test_deref_non_panic() {
    let test_value = 42;
    let test_struct = TestStruct::new(test_value);
    let deref_value = test_struct.deref();
    assert_eq!(*deref_value, test_value);
}

#[test]
fn test_deref_negative_value() {
    let test_value = -1;
    let test_struct = TestStruct::new(test_value);
    let deref_value = test_struct.deref();
    assert_eq!(*deref_value, test_value);
}

#[test]
fn test_deref_zero_value() {
    let test_value = 0;
    let test_struct = TestStruct::new(test_value);
    let deref_value = test_struct.deref();
    assert_eq!(*deref_value, test_value);
}

