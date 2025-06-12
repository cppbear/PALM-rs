// Answer 0

#[test]
fn test_remove_occupied_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    assert!(set.is_empty() && set.capacity() == 0);

    set.insert("poneyland");
    let capacity_before_remove = set.capacity();

    if let Entry::Occupied(o) = set.entry("poneyland") {
        assert_eq!(o.remove(), "poneyland");
    }

    assert_eq!(set.contains("poneyland"), false);
    assert!(set.len() == 0 && set.capacity() == capacity_before_remove);
}

#[test]
#[should_panic]
fn test_remove_non_occupied_entry() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    
    if let Entry::Occupied(o) = set.entry("nonexistent") {
        o.remove();  // This should panic since the entry does not exist
    }
}

