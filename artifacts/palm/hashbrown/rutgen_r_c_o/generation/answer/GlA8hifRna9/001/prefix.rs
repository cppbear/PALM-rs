// Answer 0

#[test]
fn test_find_entry_success() {
    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = |val: &(u64, &str)| val.0;
    table.insert_unique(1, (1, "entry1"), |val| hasher(val));

    let result = table.find_entry(1, |val| val.0 == 1);
}

#[test]
fn test_find_entry_success_multiple_entries() {
    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = |val: &(u64, &str)| val.0;
    table.insert_unique(2, (2, "entry2"), |val| hasher(val));
    table.insert_unique(3, (3, "entry3"), |val| hasher(val));
    table.insert_unique(1, (1, "entry1"), |val| hasher(val));

    let result = table.find_entry(1, |val| val.0 == 1);
}

#[test]
fn test_find_entry_no_match() {
    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = |val: &(u64, &str)| val.0;
    table.insert_unique(2, (2, "entry2"), |val| hasher(val));

    let result = table.find_entry(3, |val| val.0 == 1);
}

#[test]
fn test_find_entry_edge_case() {
    let mut table: HashTable<(u64, &str)> = HashTable::new_in(Global);
    let hasher = |val: &(u64, &str)| val.0;

    table.insert_unique(1, (1, "entry1"), |val| hasher(val));
    let result = table.find_entry(1, |val| val.0 == 1);
    let result2 = table.find_entry(1, |val| val.0 != 2); 
}

