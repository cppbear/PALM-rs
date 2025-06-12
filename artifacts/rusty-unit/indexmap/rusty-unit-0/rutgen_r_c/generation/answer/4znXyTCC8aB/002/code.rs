// Answer 0

#[test]
fn test_shift_insert_unique_at_end() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::with_capacity(1);
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Fill the entries to just below capacity
    entries.push(Bucket { hash: HashValue(1), key: 0, value: 10 });
    
    // Index equals to current length (boundary condition)
    let index = entries.len();
    let hash = HashValue(2);
    let key = 1;
    let value = 20;

    ref_mut.shift_insert_unique(index, hash, key, value);

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[1].key, key);
    assert_eq!(entries[1].value, value);
}

#[test]
#[should_panic]
fn test_shift_insert_unique_panic_on_full_capacity() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::with_capacity(1);
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Fill the entries to capacity
    entries.push(Bucket { hash: HashValue(1), key: 0, value: 10 });
    let index = entries.len();
    let hash = HashValue(2);
    let key = 1;
    let value = 20;

    // Manually set capacity to current length for testing panic
    entries.reserve(1);
    ref_mut.shift_insert_unique(index, hash, key, value); // This should panic
}

#[test]
fn test_shift_insert_unique_with_non_filled_capacity() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::with_capacity(2);
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Fill one entry
    entries.push(Bucket { hash: HashValue(1), key: 0, value: 10 });

    // Index is within current length (not at capacity)
    let index = entries.len();
    let hash = HashValue(2);
    let key = 1;
    let value = 20;

    ref_mut.shift_insert_unique(index, hash, key, value);

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[1].key, key);
    assert_eq!(entries[1].value, value);
}

