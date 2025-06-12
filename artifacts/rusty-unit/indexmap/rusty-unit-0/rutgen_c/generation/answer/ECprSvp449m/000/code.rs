// Answer 0

#[test]
fn test_iter_new_with_non_empty_entries() {
    struct TestBucket {
        hash: u64,
        key: i32,
        value: String,
    }

    let entries = [
        Bucket {
            hash: 1,
            key: 10,
            value: TestBucket {
                hash: 1,
                key: 10,
                value: "value1".to_string(),
            },
        },
        Bucket {
            hash: 2,
            key: 20,
            value: TestBucket {
                hash: 2,
                key: 20,
                value: "value2".to_string(),
            },
        },
    ];

    let iter = Iter::new(&entries);
    assert_eq!(iter.iter.len(), 2);
}

#[test]
fn test_iter_new_with_empty_entries() {
    let entries: Vec<Bucket<i32, String>> = Vec::new();
    
    let iter = Iter::new(&entries);
    assert_eq!(iter.iter.len(), 0);
}

