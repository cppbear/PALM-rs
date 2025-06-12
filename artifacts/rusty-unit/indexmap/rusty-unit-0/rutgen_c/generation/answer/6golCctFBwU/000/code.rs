// Answer 0

#[test]
fn test_iter_new_with_non_empty_entries() {
    struct HashValue(usize); // a simple struct for testing purposes

    let entries = [
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
    ];
    
    let iter = Iter::new(&entries);

    assert_eq!(iter.iter.len(), 2);
}

#[test]
fn test_iter_new_with_empty_entries() {
    struct HashValue(usize); // a simple struct for testing purposes

    let entries: Vec<Bucket<&str, &str>> = Vec::new();
    
    let iter = Iter::new(&entries);

    assert_eq!(iter.iter.len(), 0);
}

