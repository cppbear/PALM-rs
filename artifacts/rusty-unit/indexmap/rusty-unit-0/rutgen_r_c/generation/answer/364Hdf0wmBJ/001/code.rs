// Answer 0

#[test]
fn test_insert_unique_non_empty_entries() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, String>> = Vec::new();
    let entries_capacity = 2; // Ensuring capacity constraint
    entries.reserve(entries_capacity);
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // Pre-fill entries to satisfy the maximum capacity condition
    ref_mut.entries.push(Bucket {
        hash: HashValue(1),
        key: 1,
        value: "Value1".to_string(),
    });
    ref_mut.entries.push(Bucket {
        hash: HashValue(2),
        key: 2,
        value: "Value2".to_string(),
    });
    
    // Now entries.len() == entries.capacity(), this setup invokes the insert_unique
    let occupied_entry = ref_mut.insert_unique(HashValue(3), 3, "Value3".to_string());
    
    assert_eq!(occupied_entry.key(), &3);
    assert_eq!(occupied_entry.get(), &"Value3".to_string());
    assert_eq!(ref_mut.entries.len(), 3);
    assert_eq!(ref_mut.entries.capacity(), entries_capacity);
}

#[test]
#[should_panic]
fn test_insert_unique_panic_on_exceeding_capacity() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, String>> = Vec::new();
    let entries_capacity = 2; // Ensuring capacity constraint
    entries.reserve(entries_capacity);
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // Pre-fill entries to satisfy the maximum capacity condition
    ref_mut.entries.push(Bucket {
        hash: HashValue(1),
        key: 1,
        value: "Value1".to_string(),
    });
    ref_mut.entries.push(Bucket {
        hash: HashValue(2),
        key: 2,
        value: "Value2".to_string(),
    });
    
    // This call should panic as we're inserting without capacity
    ref_mut.insert_unique(HashValue(3), 3, "Value3".to_string());
}

