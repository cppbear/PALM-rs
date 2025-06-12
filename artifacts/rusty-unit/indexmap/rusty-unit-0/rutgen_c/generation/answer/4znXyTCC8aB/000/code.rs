// Answer 0

#[test]
fn test_shift_insert_unique_at_start() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.shift_insert_unique(0, HashValue(1), 10, 100);
    
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 10);
    assert_eq!(entries[0].value, 100);
}

#[test]
fn test_shift_insert_unique_at_end() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.shift_insert_unique(0, HashValue(1), 20, 200);
    ref_mut.shift_insert_unique(1, HashValue(2), 30, 300);
    
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[1].key, 30);
    assert_eq!(entries[1].value, 300);
}

#[test]
fn test_shift_insert_unique_in_middle() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.shift_insert_unique(0, HashValue(1), 10, 100);
    ref_mut.shift_insert_unique(1, HashValue(2), 20, 200);
    ref_mut.shift_insert_unique(1, HashValue(3), 15, 150);
    
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[1].key, 15);
    assert_eq!(entries[1].value, 150);
    assert_eq!(entries[2].key, 20);
    assert_eq!(entries[2].value, 200);
}

#[test]
#[should_panic]
fn test_shift_insert_unique_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.shift_insert_unique(1, HashValue(1), 10, 100); // Should panic, as current size is 0
}

