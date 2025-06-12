// Answer 0

#[test]
fn test_entry_get_occupied() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();
    set.entry("poneyland").or_insert();

    match set.entry("poneyland") {
        Entry::Vacant(_) => panic!("Expected occupied entry"),
        Entry::Occupied(entry) => assert_eq!(entry.get(), &"poneyland"),
    }
}

#[test]
fn test_entry_get_vacant() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();

    match set.entry("not_in_set") {
        Entry::Vacant(_) => {}, // Expecting a vacant entry
        Entry::Occupied(_) => panic!("Expected vacant entry"),
    }
}

