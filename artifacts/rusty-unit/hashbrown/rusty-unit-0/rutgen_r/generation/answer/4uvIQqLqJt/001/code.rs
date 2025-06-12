// Answer 0

#[test]
fn test_entry_get_occupied() {
    use hashbrown::HashSet;
    use hashbrown::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    let entry = Entry::Occupied(set.entry("poneyland"));
    assert_eq!(entry.get(), &"poneyland");
}

#[test]
fn test_entry_get_vacant() {
    use hashbrown::HashSet;
    use hashbrown::Entry;

    let mut set: HashSet<&str> = HashSet::new();
    let entry = Entry::Vacant(set.entry("horseland"));

    assert_eq!(entry.get(), &"horseland");
}

