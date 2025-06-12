// Answer 0

#[test]
fn test_vacant_entry_get() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    let vacant_entry = set.entry("poneyland");

    assert_eq!(vacant_entry.get(), &"poneyland");
}

#[test]
fn test_vacant_entry_get_empty() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    let vacant_entry = set.entry("unicornland");

    assert_eq!(vacant_entry.get(), &"unicornland");
}

