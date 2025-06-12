// Answer 0

#[test]
fn test_entry_ref_occupied() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("key1".to_string(), 1);
    
    let counter = map.entry_ref("key1").or_insert(0);
    *counter += 1;

    assert_eq!(map["key1"], 2);
}

#[test]
fn test_entry_ref_vacant() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    
    let counter = map.entry_ref("key2").or_insert(0);
    *counter += 1;

    assert_eq!(map["key2"], 1);
}

#[test]
fn test_entry_ref_multiple_inserts() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    let source = ["poneyland", "horseyland", "poneyland", "poneyland"];
    for &s in source.iter() {
        let counter = map.entry_ref(s).or_insert(0);
        *counter += 1;
    }

    assert_eq!(map["poneyland"], 3);
    assert_eq!(map["horseyland"], 1);
}

#[test]
fn test_entry_ref_boundary_condition() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    
    // Inserting a large number of elements
    for i in 0..1000 {
        let key = format!("key{}", i);
        let counter = map.entry_ref(&key).or_insert(0);
        *counter += 1;
    }

    assert_eq!(map["key999"], 1);
    assert_eq!(map.get("key1000"), None);
}

