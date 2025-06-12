// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::hash_set::{HashSet, Entry};
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<&str, RandomState> = HashSet::new();
    set.insert("horseyland");

    let entry = set.entry("horseyland").insert();

    assert_eq!(entry.get(), &"horseyland");
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::hash_set::{HashSet, Entry};
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<&str, RandomState> = HashSet::new();

    let entry = set.entry("newland").insert();

    assert_eq!(entry.get(), &"newland");
} 

#[test]
#[should_panic]
fn test_insert_on_occupied_entry() {
    use hashbrown::hash_set::{HashSet, Entry};
    use std::collections::hash_map::RandomState;

    let mut set: HashSet<&str, RandomState> = HashSet::new();
    set.insert("occupied");

    let entry = set.entry("occupied").insert();

    assert_eq!(entry.get(), &"occupied");
    
    // Trying to reinsert into occupied entry, should not panic but this is to demonstrate an invalid scenario.
    let _entry_again = set.entry("occupied").insert(); // In reality this should do nothing or panic.
}

