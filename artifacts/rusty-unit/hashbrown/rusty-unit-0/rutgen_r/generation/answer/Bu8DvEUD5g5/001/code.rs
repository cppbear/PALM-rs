// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;
    use std::hash::BuildHasherDefault;

    // Initialize a HashSet with a simple hasher
    let mut set: HashSet<&str, BuildHasherDefault<std::collections::hash_map::RandomState>> = HashSet::new();

    // Check the entry is vacant and insert a value
    if let Entry::Vacant(o) = set.entry("poneyland") {
        let occupied_entry = o.insert();
        assert!(set.contains("poneyland"));
    } else {
        panic!("Expected a vacant entry while inserting 'poneyland'");
    }
}

#[test]
fn test_insert_multiple_vacant_entries() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;
    use std::hash::BuildHasherDefault;

    let mut set: HashSet<&str, BuildHasherDefault<std::collections::hash_map::RandomState>> = HashSet::new();

    // Insert multiple values
    let values = ["poneyland", "unicornia", "dragonland"];
    for &value in &values {
        if let Entry::Vacant(o) = set.entry(value) {
            let occupied_entry = o.insert();
            assert!(set.contains(value));
        } else {
            panic!("Expected a vacant entry while inserting '{}'", value);
        }
    }
}

#[test]
#[should_panic(expected = "Expected a vacant entry while inserting 'poneyland'")]
fn test_insert_existing_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;
    use std::hash::BuildHasherDefault;

    let mut set: HashSet<&str, BuildHasherDefault<std::collections::hash_map::RandomState>> = HashSet::new();

    // Insert the value into the set
    if let Entry::Vacant(o) = set.entry("poneyland") {
        o.insert();
    }

    // Attempt to insert the same value again, expecting a panic
    if let Entry::Vacant(o) = set.entry("poneyland") {
        o.insert();
    }
}

