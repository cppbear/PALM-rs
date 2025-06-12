// Answer 0

#[test]
fn test_or_insert_with_empty_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let (key, value) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 3);
    assert_eq!(*key, "poneyland");
    assert_eq!(*value, 3);
}

#[test]
fn test_or_insert_with_existing_entry() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 3);
    
    let (_, value) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 10);
    *value *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_different_default() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    {
        let (key, value) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 10);
        assert_eq!(*key, "poneyland");
        assert_eq!(*value, 10);
    }
    
    {
        let (_, value) = map.raw_entry_mut().from_key("poneyland").or_insert("poneyland", 20);
        *value *= 3;
    }
    
    assert_eq!(map["poneyland"], 30);
}

