// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use hashbrown::HashMap;
    use hashbrown::entry::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 41);

    let entry = Entry::Occupied(map.entry("poneyland"));
    let modified_entry = entry.and_modify(|e| { *e += 1 });

    if let Entry::Occupied(occupied) = modified_entry {
        assert_eq!(*occupied.get(), 42);
    } else {
        panic!("Expected Entry::Occupied but got Entry::Vacant");
    }
}

#[test]
fn test_and_modify_with_vacant_entry() {
    use hashbrown::HashMap;
    use hashbrown::entry::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    let entry = Entry::Vacant(map.entry("unicornland"));
    let modified_entry = entry.and_modify(|e| { *e += 1 });

    if let Entry::Vacant(_) = modified_entry {
        // This is the expected outcome
    } else {
        panic!("Expected Entry::Vacant but got Entry::Occupied");
    }
}

#[test]
fn test_and_modify_multiple_modifications() {
    use hashbrown::HashMap;
    use hashbrown::entry::Entry;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 41);

    let entry = Entry::Occupied(map.entry("poneyland"));
    let modified_entry = entry.and_modify(|e| { *e += 1 });
    if let Entry::Occupied(occupied) = modified_entry {
        assert_eq!(*occupied.get(), 42);
    }
    
    let modified_entry_second = map.entry("poneyland").and_modify(|e| { *e += 1 });
    if let Entry::Occupied(occupied) = modified_entry_second {
        assert_eq!(*occupied.get(), 43);
    }
}

