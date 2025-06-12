// Answer 0

#[test]
fn test_capacity_empty() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::new();
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_capacity_with_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    assert_eq!(map.capacity(), 10);
}

#[test]
fn test_capacity_after_inserting_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    // Simulating insertion by manipulating entries and indices directly
    map.entries.push(Bucket {
        hash: HashValue::default(), 
        key: 1,
        value: 100,
    });
    // Assuming a single Bucket means one entry has been added
    assert_eq!(map.capacity(), 1);
}

#[test]
fn test_capacity_with_indices() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    // Again fixing the indices, as we don't have an actual insertion method
    map.indices = Indices::with_capacity(3);
    assert_eq!(map.capacity(), 3);
}

#[test]
fn test_capacity_max_capacity() {
    let map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
    assert_eq!(map.capacity(), IndexMapCore::<usize, usize>::MAX_ENTRIES_CAPACITY);
}

