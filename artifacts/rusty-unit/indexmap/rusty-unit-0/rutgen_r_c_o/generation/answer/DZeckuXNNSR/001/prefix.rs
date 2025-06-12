// Answer 0

#[test]
fn test_update_index_existing_entry() {
    let mut table = hash_table::HashTable::new();
    let hash = HashValue(5);
    table.insert(hash.get(), 10);
    update_index(&mut table, hash, 10, 20);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_update_index_non_existing_old_value() {
    let mut table = hash_table::HashTable::new();
    let hash = HashValue(3);
    table.insert(hash.get(), 15);
    update_index(&mut table, hash, 10, 25);
}

#[test]
fn test_update_index_boundary_values() {
    let mut table = hash_table::HashTable::new();
    let hash_min = HashValue(0);
    let hash_max = HashValue(10_000_000_000);
    
    table.insert(hash_min.get(), 0);
    update_index(&mut table, hash_min, 0, 1);
    
    table.insert(hash_max.get(), 10_000_000_000);
    update_index(&mut table, hash_max, 10_000_000_000, 5);
}

#[test]
fn test_update_index_duplicate_entries() {
    let mut table = hash_table::HashTable::new();
    let hash = HashValue(2);
    table.insert(hash.get(), 50);
    table.insert(hash.get(), 50);
    update_index(&mut table, hash, 50, 100);
}

