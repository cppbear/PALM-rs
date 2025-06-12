// Answer 0

#[test]
fn test_get_index_of_found() {
    struct TestKey;
    
    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, other: &usize) -> bool {
            // Define equivalent logic as needed, can be based on certain conditions
            *other == 42 // example condition
        }
    }

    let mut index_map = IndexMapCore::new();
    let hash_value = HashValue(1);
    
    // Manually insert an entry with key 42
    index_map.entries.push(Bucket {
        hash: hash_value,
        key: 42,
        value: "value",
    });
    
    index_map.indices.insert(hash_value.get(), 0); // insert index for the hash

    let index = index_map.get_index_of(hash_value, &TestKey);
    assert_eq!(index, Some(0)); // should find the key at index 0
}

#[test]
fn test_get_index_of_not_found() {
    struct TestKey;

    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, other: &usize) -> bool {
            *other == 42 // example condition
        }
    }

    let index_map = IndexMapCore::new();
    let hash_value = HashValue(1);

    // No entries have been inserted, should return None
    let index = index_map.get_index_of(hash_value, &TestKey);
    assert_eq!(index, None); // no entry to find
}

#[test]
fn test_get_index_of_multiple_entries() {
    struct TestKey;

    impl Equivalent<usize> for TestKey {
        fn equivalent(&self, other: &usize) -> bool {
            *other == 42
        }
    }

    let mut index_map = IndexMapCore::new();
    let hash_value_1 = HashValue(1);
    let hash_value_2 = HashValue(2);

    // Insert multiple entries
    index_map.entries.push(Bucket {
        hash: hash_value_1,
        key: 42,
        value: "first_value",
    });
    
    index_map.entries.push(Bucket {
        hash: hash_value_2,
        key: 43,
        value: "second_value",
    });

    index_map.indices.insert(hash_value_1.get(), 0); // entry for 42 at index 0
    index_map.indices.insert(hash_value_2.get(), 1); // entry for 43 at index 1

    let index = index_map.get_index_of(hash_value_1, &TestKey);
    assert_eq!(index, Some(0)); // should find the key at index 0
    
    let index_not_found = index_map.get_index_of(hash_value_2, &TestKey);
    assert_eq!(index_not_found, None); // should not find TestKey in hash_value_2 entry
}

