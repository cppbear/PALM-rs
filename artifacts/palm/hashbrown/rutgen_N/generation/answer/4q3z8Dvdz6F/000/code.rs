// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    map.raw_entry_mut()
        .from_key("poneyland")
        .and_modify(|_k, v| { *v += 1 })
        .or_insert("poneyland", 42);
    assert_eq!(map["poneyland"], 42);

    map.raw_entry_mut()
        .from_key("poneyland")
        .and_modify(|_k, v| { *v += 1 })
        .or_insert("poneyland", 0);
    assert_eq!(map["poneyland"], 43);
}

#[test]
fn test_and_modify_with_vacant_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    map.raw_entry_mut()
        .from_key("vacantland")
        .and_modify(|_k, v| { *v += 1 })
        .or_insert("vacantland", 15);
    assert_eq!(map["vacantland"], 15);
    
    // Trying to modify a vacant entry should not change the value
    map.raw_entry_mut()
        .from_key("vacantland")
        .and_modify(|_k, v| { *v += 1 })
        .or_insert("vacantland", 0);
    assert_eq!(map["vacantland"], 15);
}

#[test]
#[should_panic]
fn test_and_modify_panic_on_unoccupied_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    
    map.raw_entry_mut()
        .from_key("nonexistent")
        .and_modify(|_k, v| { *v += 1 })
        .or_insert("nonexistent", 10); // This will insert the value

    map.raw_entry_mut()
        .from_key("another_nonexistent")
        .and_modify(|_k, v| { *v += 1 }) // This will cause panic
        .or_insert("another_nonexistent", 20);
}

