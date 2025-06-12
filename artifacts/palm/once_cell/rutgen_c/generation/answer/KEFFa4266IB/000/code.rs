// Answer 0

#[test]
fn test_once_ref_set_success() {
    struct TestData {
        value: i32,
    }

    let cell: OnceRef<TestData> = OnceRef::new();
    let data = TestData { value: 42 };
    
    let result = cell.set(&data);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_once_ref_set_failure() {
    struct TestData {
        value: i32,
    }

    let cell: OnceRef<TestData> = OnceRef::new();
    let data1 = TestData { value: 42 };
    let data2 = TestData { value: 50 };

    let result_set_first = cell.set(&data1);
    assert_eq!(result_set_first, Ok(()));

    let result_set_second = cell.set(&data2);
    assert_eq!(result_set_second, Err(()));
}

