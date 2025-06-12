// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 42);

    map.entry_ref("poneyland")
        .and_modify(|e| { *e += 1 })
        .or_insert(0);
    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_modify_with_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();

    map.entry_ref("unicornia")
        .and_modify(|e| { *e += 1 })
        .or_insert(42);
    assert_eq!(map["unicornia"], 42);
}

#[test]
fn test_and_modify_empty_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();

    let entry = map.entry_ref("dragonland");
    entry.and_modify(|e| { *e += 1 }).or_insert(32);
    assert_eq!(map["dragonland"], 32);
}

#[test]
#[should_panic]
fn test_and_modify_with_panic() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, u32> = HashMap::new();
    
    // This should panic since "notfound" entry is vacant
    map.entry_ref("notfound")
        .and_modify(|_e| panic!("Should not modify"))
        .or_insert(24);
}

