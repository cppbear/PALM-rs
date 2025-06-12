// Answer 0

#[test]
fn test_last_non_empty() {
    struct TestKey;
    struct TestValue;

    let bucket = Bucket {
        hash: HashValue::default(),
        key: TestKey,
        value: TestValue,
    };

    let slice = Slice {
        entries: [bucket],
    };

    assert!(slice.last().is_some());
}

#[test]
fn test_last_empty() {
    struct TestKey;
    struct TestValue;

    let slice = Slice {
        entries: [],
    };

    assert!(slice.last().is_none());
}

#[test]
fn test_last_multiple_entries() {
    struct TestKey;
    struct TestValue;

    let buckets = [
        Bucket {
            hash: HashValue::default(),
            key: TestKey,
            value: TestValue,
        },
        Bucket {
            hash: HashValue::default(),
            key: TestKey,
            value: TestValue,
        },
        Bucket {
            hash: HashValue::default(),
            key: TestKey,
            value: TestValue,
        },
    ];

    let slice = Slice {
        entries: buckets,
    };

    assert!(slice.last().is_some());
}

