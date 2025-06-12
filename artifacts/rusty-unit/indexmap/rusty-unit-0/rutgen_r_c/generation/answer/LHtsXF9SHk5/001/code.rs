// Answer 0

#[test]
fn test_values_new_with_non_empty_entries() {
    // Setup: Create a bucket array with sample key-value pairs.
    let entries = [
        Bucket { hash: HashValue::new(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue::new(2), key: "key2", value: "value2" },
    ];
    
    // Execute: Instantiate Values with non-empty entries.
    let values = Values::new(&entries);
    
    // Assertion: Check that the iterator can iterate over the two buckets.
    let collected: Vec<_> = values.iter.clone().collect();
    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].key, "key1");
    assert_eq!(collected[1].key, "key2");
}

#[test]
fn test_values_new_with_empty_entries() {
    // Setup: Create an empty bucket array.
    let entries: Vec<Bucket<&str, &str>> = Vec::new();
    
    // Execute: Instantiate Values with empty entries.
    let values = Values::new(&entries);
    
    // Assertion: Ensure the iterator is empty.
    let collected: Vec<_> = values.iter.clone().collect();
    assert_eq!(collected.len(), 0);
}

#[test]
#[should_panic]
fn test_values_new_with_null_entries() {
    // Setup: Attempt to create Values with null reference (this simulates a panic).
    let entries: &[Bucket<&str, &str>] = std::ptr::null();
    
    // Execute: This should panic.
    let _values = Values::new(entries);
}

