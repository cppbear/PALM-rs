// Answer 0

#[test]
fn test_len_empty_slice() {
    struct TestMap<K, V> {
        entries: [Bucket<K, V>; 0],
    }

    let slice: &Slice<i32, i32> = &Slice { entries: [] };
    assert_eq!(slice.len(), 0);
}

#[test]
fn test_len_non_empty_slice() {
    struct TestMap<K, V> {
        entries: [Bucket<K, V>; 3],
    }

    let entries = [
        Bucket { hash: 0, key: 1, value: 10 },
        Bucket { hash: 0, key: 2, value: 20 },
        Bucket { hash: 0, key: 3, value: 30 },
    ];
    let slice: &Slice<i32, i32> = &Slice { entries };
    assert_eq!(slice.len(), 3);
}

