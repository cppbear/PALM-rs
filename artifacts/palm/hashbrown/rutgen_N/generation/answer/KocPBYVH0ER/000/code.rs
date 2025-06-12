// Answer 0

#[test]
fn test_or_insert_vacant() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();

    // Inserting a nonexistent key
    set.entry("poneyland").or_insert();
    assert!(set.contains("poneyland"));
    assert_eq!(set.len(), 1);
}

#[test]
fn test_or_insert_existing() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();

    // First insertion
    set.entry("poneyland").or_insert();
    // Insert again
    set.entry("poneyland").or_insert();
    assert!(set.contains("poneyland"));
    assert_eq!(set.len(), 1);
}

