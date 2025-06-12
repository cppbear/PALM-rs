// Answer 0

#[test]
fn test_entry_occupied() {
    use hashbrown::{HashTable, DefaultHashBuilder, Entry};

    // Create a new hash table
    let mut table = HashTable::<(u32, &str)>::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &(u32, &str)| hasher.hash_one(val);

    // Insert an element
    table.insert_unique(hasher_fn(&(1, "a")), (1, "a"), hasher_fn);

    // Define equality function
    let eq_fn = |val: &(u32, &str)| val.0 == 1;

    // Get the entry for the existing element
    match table.entry(hasher_fn(&(1, "a")), eq_fn, hasher_fn) {
        Entry::Occupied(entry) => {
            // Successfully retrieved an occupied entry
            assert_eq!(entry.bucket.ptr.as_ref(), &(1, "a"));
        },
        _ => panic!("Expected an occupied entry, but got vacant."),
    }
}

#[test]
fn test_entry_vacant() {
    use hashbrown::{HashTable, DefaultHashBuilder, Entry};

    // Create a new hash table
    let mut table = HashTable::<(u32, &str)>::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &(u32, &str)| hasher.hash_one(val);

    // Insert an element
    table.insert_unique(hasher_fn(&(1, "a")), (1, "a"), hasher_fn);

    // Define equality function for a non-existing element
    let eq_fn_two = |val: &(u32, &str)| val.0 == 2;

    // Get the entry for a new element
    match table.entry(hasher_fn(&(2, "b")), eq_fn_two, hasher_fn) {
        Entry::Vacant(entry) => {
            // Successfully retrieved a vacant entry
            entry.insert((2, "b"));
            assert_eq!(table.find(hasher_fn(&(2, "b")), |val| val.0 == 2).unwrap(), &(2, "b"));
        },
        _ => panic!("Expected a vacant entry, but got occupied."),
    }
}

