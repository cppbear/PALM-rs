// Answer 0

#[test]
fn test_set_success() {
    struct TestData {
        value: i32,
    }

    let data = TestData { value: 42 };
    let once_ref = OnceRef::new();

    // Assert that the state is empty before setting the value
    assert!(once_ref.set(&data).is_ok());
    
    // Try setting the value again which should fail as the cell is now full
    assert!(once_ref.set(&data).is_err());
}

#[test]
fn test_set_multiple_success() {
    struct TestData {
        value: i32,
    }

    let data1 = TestData { value: 1 };
    let data2 = TestData { value: 2 };
    let once_ref = OnceRef::new();

    // Setting first value should succeed
    assert!(once_ref.set(&data1).is_ok());

    // Attempting to set a different value should fail
    assert!(once_ref.set(&data2).is_err());
}

#[test]
fn test_set_edge_case() {
    struct TestData {
        value: i32,
    }

    let data = TestData { value: 0 };
    let once_ref = OnceRef::new();

    // Setting a value of zero should succeed
    assert!(once_ref.set(&data).is_ok());

    // Attempting to set the same value again should fail
    assert!(once_ref.set(&data).is_err());
}

