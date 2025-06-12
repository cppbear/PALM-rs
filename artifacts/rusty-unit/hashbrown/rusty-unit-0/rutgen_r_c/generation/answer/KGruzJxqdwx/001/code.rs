// Answer 0

#[test]
fn test_fmt_vacant_entry() {
    use hashbrown::hash_table::{Entry, VacantEntry};
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a simple hash table
    let mut table = HashTable::<&str>::new();
    let hasher = DefaultHashBuilder::default();
    let key = "key";

    // Create a VacantEntry
    let vacant_entry = table.entry(hasher.hash_one(&key), |&x| x == key, hasher);

    // Match on the Entry variant
    if let Entry::Vacant(ref v) = vacant_entry {
        let mut output = String::new();
        let mut formatter = fmt::Formatter::new(&mut output);
        assert!(v.fmt(&mut formatter).is_ok());
        assert!(output.contains("Entry"));
    } else {
        panic!("Expected VacantEntry but found OccupiedEntry.");
    }
}

#[test]
fn test_fmt_occupied_entry() {
    use hashbrown::hash_table::{Entry, OccupiedEntry};
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    // Create a simple hash table
    let mut table = HashTable::<&str>::new();
    let hasher = DefaultHashBuilder::default();
    let key = "key";
    let value = "value";

    // Insert an element to create an OccupiedEntry
    table.insert_unique(hasher.hash_one(&key), value, hasher);
    
    // Create an occupied entry
    let occupied_entry = table.entry(hasher.hash_one(&key), |&x| x == key, hasher);

    // Match on the Entry variant
    if let Entry::Occupied(ref o) = occupied_entry {
        let mut output = String::new();
        let mut formatter = fmt::Formatter::new(&mut output);
        assert!(o.fmt(&mut formatter).is_ok());
        assert!(output.contains("Entry"));
    } else {
        panic!("Expected OccupiedEntry but found VacantEntry.");
    }
}

