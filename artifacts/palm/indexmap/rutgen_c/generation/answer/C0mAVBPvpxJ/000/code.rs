// Answer 0

#[test]
fn test_value_ref() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    let bucket = Bucket {
        hash: HashValue(1),
        key: TestKey,
        value: TestValue { data: 42 },
    };

    assert_eq!(bucket.value_ref().data, 42);
}

#[test]
fn test_value_ref_multiple_instances() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    let bucket1 = Bucket {
        hash: HashValue(2),
        key: TestKey,
        value: TestValue { data: 100 },
    };

    let bucket2 = Bucket {
        hash: HashValue(3),
        key: TestKey,
        value: TestValue { data: 200 },
    };

    assert_eq!(bucket1.value_ref().data, 100);
    assert_eq!(bucket2.value_ref().data, 200);
}

