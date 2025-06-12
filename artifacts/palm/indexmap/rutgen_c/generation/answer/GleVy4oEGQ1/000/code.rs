// Answer 0

#[test]
fn test_bucket_key() {
    struct TestBucket {
        key: i32,
        value: &'static str,
    }

    let bucket = Bucket {
        hash: HashValue(1),
        key: 42,
        value: "value_example",
    };

    assert_eq!(bucket.key(), 42);
}

#[test]
fn test_bucket_key_multiple_instances() {
    struct TestBucket {
        key: i32,
        value: &'static str,
    }

    let bucket1 = Bucket {
        hash: HashValue(2),
        key: 10,
        value: "first_value",
    };

    let bucket2 = Bucket {
        hash: HashValue(3),
        key: 20,
        value: "second_value",
    };

    assert_eq!(bucket1.key(), 10);
    assert_eq!(bucket2.key(), 20);
}

