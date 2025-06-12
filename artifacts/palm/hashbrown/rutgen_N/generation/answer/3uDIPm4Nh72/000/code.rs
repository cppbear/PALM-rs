// Answer 0

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    let entry = set.entry("vacant_entry").insert();

    assert_eq!(entry.get(), &"vacant_entry");
}

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    let entry1 = set.entry("occupied_entry").insert();
    let entry2 = set.entry("occupied_entry").insert();

    assert_eq!(entry1.get(), &"occupied_entry");
    assert_eq!(entry2.get(), &"occupied_entry");
}

