// Answer 0

#[test]
fn test_remove_from_occupied_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    assert!(set.is_empty() && set.capacity() == 0);

    set.entry("poneyland").or_insert();
    let capacity_before_remove = set.capacity();

    if let Entry::Occupied(o) = set.entry("poneyland") {
        assert_eq!(o.remove(), "poneyland");
    }

    assert_eq!(set.contains("poneyland"), false);
    assert!(set.len() == 0 && set.capacity() == capacity_before_remove);
}

#[test]
fn test_remove_non_existent_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    assert!(set.is_empty() && set.capacity() == 0);

    if let Entry::Vacant(v) = set.entry("nonexistent") {
        // This will not panic since there is no entry to remove
        let entry_ref = v.insert();
        assert_eq!(set.len(), 1);
        assert_eq!(entry_ref, &mut "nonexistent");
    }
    
    assert_eq!(set.contains("nonexistent"), true);
    if let Entry::Occupied(o) = set.entry("nonexistent") {
        assert_eq!(o.remove(), "nonexistent");
    }

    assert_eq!(set.contains("nonexistent"), false);
    assert!(set.len() == 0);
}

