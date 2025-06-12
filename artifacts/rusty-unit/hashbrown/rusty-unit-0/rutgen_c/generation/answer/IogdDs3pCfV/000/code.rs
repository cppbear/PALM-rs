// Answer 0

#[test]
fn test_iter_hash_empty() {
    use hashbrown::HashTable;
    
    let table: HashTable<&str> = HashTable::new_in(Global);
    let hasher = |val: &str| val.len() as u64;

    let result: Vec<_> = table.iter_hash(hasher("a")).collect();
    assert!(result.is_empty());
}

#[test]
fn test_iter_hash_single_element() {
    use hashbrown::HashTable;

    let mut table: HashTable<&str> = HashTable::new_in(Global);
    let hasher = |val: &str| val.len() as u64;
    
    table.insert_unique(hasher("a"), "a", hasher);
    
    let result: Vec<_> = table.iter_hash(hasher("a")).collect();
    assert_eq!(result, vec!["a"]);
}

#[test]
fn test_iter_hash_multiple_elements_with_same_hash() {
    use hashbrown::HashTable;

    let mut table: HashTable<&str> = HashTable::new_in(Global);
    let hasher = |val: &str| val.len() as u64;
    
    table.insert_unique(hasher("a"), "a", hasher);
    table.insert_unique(hasher("b"), "b", hasher);
    
    let result: Vec<_> = table.iter_hash(hasher("a")).collect();
    assert!(result.contains(&"a"));
    assert!(result.contains(&"b"));
}

#[test]
fn test_iter_hash_with_different_hash() {
    use hashbrown::HashTable;

    let mut table: HashTable<&str> = HashTable::new_in(Global);
    let hasher = |val: &str| val.len() as u64;

    table.insert_unique(hasher("one"), "one", hasher);
    table.insert_unique(hasher("two"), "two", hasher);
    
    let result: Vec<_> = table.iter_hash(hasher("three")).collect();
    assert!(result.is_empty());
}

