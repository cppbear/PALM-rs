// Answer 0

#[test]
fn test_get_range_valid() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];

    let slice = Slice { entries };
    
    let result = slice.get_range(0..2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_get_range_empty() {
    struct TestKey;
    struct TestValue;

    let entries: [Bucket<TestKey, TestValue>; 0] = [];
    let slice = Slice { entries };

    let result = slice.get_range(0..1);
    assert!(result.is_none());
}

#[test]
fn test_get_range_out_of_bounds() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
    ];

    let slice = Slice { entries };

    let result = slice.get_range(1..3);
    assert!(result.is_none());
}

#[test]
fn test_get_range_full() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];

    let slice = Slice { entries };

    let result = slice.get_range(..);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn test_get_range_inclusive() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];

    let slice = Slice { entries };

    let result = slice.get_range(1..=2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

