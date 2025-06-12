// Answer 0

#[test]
fn test_or_insert_with_occupied() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("poneyland", "hoho".to_string());

    let entry = map.raw_entry_mut().from_key("poneyland");
    let (key, value) = entry.or_insert_with(|| ("poneyland", "new_value".to_string()));

    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
}

#[test]
fn test_or_insert_with_vacant() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, String> = HashMap::new();

    let entry = map.raw_entry_mut().from_key("poneyland");
    let (key, value) = entry.or_insert_with(|| ("poneyland", "hoho".to_string()));

    assert_eq!(*key, "poneyland");
    assert_eq!(*value, "hoho".to_string());
    assert_eq!(map["poneyland"], "hoho".to_string());
}

#[test]
fn test_or_insert_with_empty_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, String> = HashMap::new();

    let entry = map.raw_entry_mut().from_key("");
    let (key, value) = entry.or_insert_with(|| (String::new(), "empty".to_string()));

    assert_eq!(*key, "");
    assert_eq!(*value, "empty".to_string());
    assert_eq!(map[""], "empty".to_string());
}

