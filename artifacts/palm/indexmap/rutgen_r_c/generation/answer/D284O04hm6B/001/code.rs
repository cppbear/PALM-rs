// Answer 0

#[test]
fn test_index_mut_valid_index() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: TestKey, value: TestValue { data: 1 } },
            Bucket { hash: 1, key: TestKey, value: TestValue { data: 2 } },
            Bucket { hash: 2, key: TestKey, value: TestValue { data: 3 } },
        ]
    };

    let value_ref = slice.index_mut(1);
    value_ref.data += 10;
    assert_eq!(slice.entries[1].value.data, 12);
}

#[test]
#[should_panic]
fn test_index_mut_out_of_bounds() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: TestKey, value: TestValue { data: 1 } },
            Bucket { hash: 1, key: TestKey, value: TestValue { data: 2 } },
        ]
    };

    let _value_ref = slice.index_mut(2); // This should panic
}

#[test]
fn test_index_mut_zero_index() {
    struct TestKey;
    struct TestValue {
        data: i32,
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: 0, key: TestKey, value: TestValue { data: 5 } },
            Bucket { hash: 1, key: TestKey, value: TestValue { data: 10 } },
        ]
    };

    let value_ref = slice.index_mut(0);
    assert_eq!(value_ref.data, 5);
    value_ref.data += 5;
    assert_eq!(slice.entries[0].value.data, 10);
}

