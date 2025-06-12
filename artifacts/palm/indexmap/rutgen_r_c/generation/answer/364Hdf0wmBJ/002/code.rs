// Answer 0

#[test]
fn test_insert_unique() {
    let mut indices: Indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();

    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let hash_value = HashValue(1);
    let key = 1;
    let value = 42;

    let occupied_entry = ref_mut.insert_unique(hash_value, key, value);

    assert_eq!(occupied_entry.key(), &key);
    assert_eq!(occupied_entry.get(), &value);
}

#[test]
fn test_insert_unique_with_capacity() {
    let mut indices: Indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::with_capacity(2);
    entries.push(Bucket { hash: HashValue(0), key: 0, value: 0 }); // Pre-filling to avoid capacity check

    let ref_mut = RefMut::new(&mut indices, &mut entries);
    let hash_value = HashValue(1);
    let key = 1;
    let value = 42;

    let occupied_entry = ref_mut.insert_unique(hash_value, key, value);

    assert_eq!(occupied_entry.key(), &key);
    assert_eq!(occupied_entry.get(), &value);
    assert_eq!(entries.len(), 2); // Ensure the entry was added
}

#[test]
#[should_panic]
fn test_insert_unique_panic_condition() {
    let mut indices: Indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::with_capacity(1);
    let ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let hash_value = HashValue(1);
    let key = 1;
    let value = 42;

    ref_mut.insert_unique(hash_value, key, value); // This should not panic
    assert_eq!(entries.len(), 1); // Ensure it was added first

    // Attempting to insert again without checking might not be able to because of length restrictions
    ref_mut.insert_unique(hash_value, key, value);
}

