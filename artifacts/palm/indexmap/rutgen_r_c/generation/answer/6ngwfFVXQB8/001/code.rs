// Answer 0

#[test]
fn test_first_mut_non_empty() {
    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: "key1", value: 10 },
            Bucket { hash: 2, key: "key2", value: 20 },
        ],
    };

    let result = slice.first_mut();
    assert!(result.is_some());
    if let Some((key, value)) = result {
        assert_eq!(key, &"key1");
        *value += 5;
        assert_eq!(*value, 15);
    }
}

#[test]
fn test_first_mut_empty() {
    let mut slice: Slice<&str, i32> = Slice { entries: [] };

    let result = slice.first_mut();
    assert!(result.is_none());
}

#[test]
fn test_first_mut_single_element() {
    let mut slice = Slice {
        entries: [Bucket { hash: 3, key: "key3", value: 30 }],
    };

    let result = slice.first_mut();
    assert!(result.is_some());
    if let Some((key, value)) = result {
        assert_eq!(key, &"key3");
        *value += 10;
        assert_eq!(*value, 40);
    }
}

