// Answer 0

#[test]
fn test_increment_indices_with_shifted_entries() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2);
    
    let entry1 = Bucket { hash: HashValue(0), key: 1, value: "value1" };
    let entry2 = Bucket { hash: HashValue(1), key: 2, value: "value2" };
    let entries = vec![entry1, entry2];

    let mut refmut = RefMut::new(&mut indices, &mut entries);

    refmut.increment_indices(0, 2);

    assert_eq!(indices.get(&HashValue(0).0), Some(&1));
    assert_eq!(indices.get(&HashValue(1).0), Some(&2));
}

#[test]
fn test_increment_indices_no_shift() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    
    let entry = Bucket { hash: HashValue(1), key: 2, value: "value" };
    let entries = vec![entry];

    let mut refmut = RefMut::new(&mut indices, &mut entries);

    refmut.increment_indices(1, 1);

    assert_eq!(indices.get(&HashValue(1).0), Some(&1));
}

#[test]
fn test_increment_indices_empty() {
    let mut indices = hash_table::HashTable::new();
    let entries: Vec<Bucket<usize, &str>> = vec![];

    let mut refmut = RefMut::new(&mut indices, &mut entries);

    refmut.increment_indices(0, 0);

    assert!(indices.is_empty());
}

