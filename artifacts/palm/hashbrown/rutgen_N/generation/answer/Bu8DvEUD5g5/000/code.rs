// Answer 0

#[test]
fn test_insert_into_vacant_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();

    if let Entry::Vacant(o) = set.entry("poneyland") {
        o.insert();
    }
    
    assert!(set.contains("poneyland"));
}

#[test]
fn test_insert_multiple_vacant_entries() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();

    if let Entry::Vacant(o) = set.entry("poneyland") {
        o.insert();
    }
    if let Entry::Vacant(o) = set.entry("unicornland") {
        o.insert();
    }

    assert!(set.contains("poneyland"));
    assert!(set.contains("unicornland"));
}

#[test]
fn test_insert_existing_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();

    if let Entry::Vacant(o) = set.entry("poneyland") {
        o.insert();
    }

    assert!(set.contains("poneyland"));
    
    // Attempt to insert again, should not panic or change the state
    if let Entry::Vacant(o) = set.entry("poneyland") {
        o.insert();
    }

    assert!(set.contains("poneyland"));
}

