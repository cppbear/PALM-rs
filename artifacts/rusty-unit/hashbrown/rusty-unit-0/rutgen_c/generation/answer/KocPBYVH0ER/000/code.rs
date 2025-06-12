// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::HashSet;
    let mut set: HashSet<&str> = HashSet::new();

    // Inserting a nonexistent key
    let entry = set.entry("poneyland");
    entry.or_insert();
    assert!(set.contains("poneyland"));

    // Re-inserting an existing key
    entry.or_insert();
    assert!(set.contains("poneyland"));
    assert_eq!(set.len(), 1);
}

#[test]
fn test_or_insert_with_different_keys() {
    use hashbrown::HashSet;
    let mut set: HashSet<i32> = HashSet::new();

    // Inserting a nonexistent key
    let entry1 = set.entry(1);
    entry1.or_insert();
    assert!(set.contains(&1));

    // Inserting another nonexistent key
    let entry2 = set.entry(2);
    entry2.or_insert();
    assert!(set.contains(&2));
    
    // Validating size
    assert_eq!(set.len(), 2);
}

