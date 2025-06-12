// Answer 0

#[test]
fn test_first_mut_non_empty() {
    struct TestSlice {
        entries: [Bucket<&'static str, i32>; 2],
    }

    let mut slice = Slice {
        entries: [
            Bucket { hash: HashValue(0), key: "a", value: 1 },
            Bucket { hash: HashValue(0), key: "b", value: 2 },
        ],
    };

    let result = slice.first_mut();
    assert!(result.is_some());
    if let Some((key, value)) = result {
        assert_eq!(key, &"a");
        *value += 1;
        assert_eq!(slice.entries[0].value, 2);
    }
}

#[test]
fn test_first_mut_empty() {
    struct TestSlice {
        entries: [Bucket<&'static str, i32>; 0],
    }

    let mut slice = Slice { entries: [] };

    let result = slice.first_mut();
    assert!(result.is_none());
}

