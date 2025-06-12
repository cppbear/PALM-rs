// Answer 0

#[test]
fn test_value_mut() {
    struct TestBucket {
        hash: HashValue,
        key: String,
        value: i32,
    }

    let mut bucket = TestBucket {
        hash: HashValue(1),
        key: String::from("key1"),
        value: 10,
    };

    // Accessing mutable reference to value
    {
        let value_mut = bucket.value_mut();
        *value_mut += 5; // change value
    }

    assert_eq!(bucket.value, 15);
}

#[test]
fn test_value_mut_initialization() {
    struct AnotherBucket {
        hash: HashValue,
        key: u32,
        value: f64,
    }

    let mut bucket = AnotherBucket {
        hash: HashValue(2),
        key: 100,
        value: 2.5,
    };

    {
        let value_mut = bucket.value_mut();
        *value_mut *= 2.0; // double the value
    }

    assert_eq!(bucket.value, 5.0);
}

