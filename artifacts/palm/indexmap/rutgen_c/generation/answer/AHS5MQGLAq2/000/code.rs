// Answer 0

#[test]
fn test_split_last_non_empty() {
    let bucket1 = Bucket { hash: 0, key: 1, value: "a" };
    let bucket2 = Bucket { hash: 0, key: 2, value: "b" };
    let slice = Slice { entries: [bucket1, bucket2] };

    if let Some((last, rest)) = slice.split_last() {
        assert_eq!(last.0, &2);
        assert_eq!(last.1, &"b");
        assert_eq!(rest.len(), 1);
        assert_eq!(rest.get_index(0), Some((&1, &"a")));
    } else {
        panic!("Expected to get last element");
    }
}

#[test]
fn test_split_last_empty() {
    let slice: Slice<i32, &str> = Slice { entries: [] };

    let result = slice.split_last();
    assert!(result.is_none());
}

