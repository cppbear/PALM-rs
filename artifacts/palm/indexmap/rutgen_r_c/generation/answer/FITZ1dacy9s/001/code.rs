// Answer 0

#[test]
fn test_split_first_non_empty() {
    struct TestBucket {
        hash: u64, // Simulating HashValue as u64 for simplicity
        key: i32,
        value: i32,
    }

    let entries = [
        Bucket { hash: 1, key: 10, value: 100 },
        Bucket { hash: 2, key: 20, value: 200 },
        Bucket { hash: 3, key: 30, value: 300 },
    ];

    let slice = Slice {
        entries: entries,
    };

    let result = slice.split_first();
    assert!(result.is_some());

    let (first_key, rest_slice) = result.unwrap();
    assert_eq!(*first_key, 10);
    assert_eq!(rest_slice.entries.len(), 2);
    assert_eq!(rest_slice.entries[0].key, 20);
}

#[test]
fn test_split_first_empty() {
    let empty_slice = Slice { entries: [] };

    let result = empty_slice.split_first();
    assert!(result.is_none());
}

