// Answer 0

#[test]
fn test_index_valid() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
    ];
    
    let slice = Slice { entries };

    assert_eq!(slice.index(0), &TestValue);
    assert_eq!(slice.index(1), &TestValue);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_out_of_bounds() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
    ];
    
    let slice = Slice { entries };

    let _ = slice.index(1); // This should panic
}

