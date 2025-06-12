// Answer 0

#[test]
fn test_get_occupied_entry() {
    use hashbrown::HashSet;
    
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    let entry = set.entry("poneyland");
    assert_eq!(entry.get(), &"poneyland");
}

#[test]
fn test_get_vacant_entry() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    
    let entry = set.entry("horseland");
    assert_eq!(entry.get(), &"horseland");
}

#[test]
fn test_get_vacant_and_insert() {
    use hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    
    {
        let entry = set.entry("horseland");
        assert_eq!(entry.get(), &"horseland");
        entry.insert(); // insert empty to turn Vacant to Occupied
    }

    assert!(set.contains("horseland"));
    assert_eq!(set.entry("horseland").get(), &"horseland");
}

