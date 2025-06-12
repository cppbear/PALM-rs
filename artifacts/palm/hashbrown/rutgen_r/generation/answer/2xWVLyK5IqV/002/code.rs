// Answer 0

#[test]
fn test_or_insert_with_key_occupied() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("poneyland".to_string(), 9);

    let entry = map.entry_ref("poneyland");
    let value = entry.or_insert_with_key(|key| key.chars().count());

    assert_eq!(*value, 9);
}

#[test]
fn test_or_insert_with_key_vacant() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();

    let entry = map.entry_ref("unicornland");
    let value = entry.or_insert_with_key(|key| key.chars().count());

    assert_eq!(*value, 12);
    assert_eq!(map["unicornland"], 12);
}

#[test]
fn test_or_insert_with_key_existing_update() {
    use hashbrown::HashMap;

    let mut map: HashMap<String, usize> = HashMap::new();
    map.insert("dragonland".to_string(), 10);

    {
        let entry = map.entry_ref("dragonland");
        *entry.or_insert_with_key(|key| key.chars().count() * 2) *= 2;
    }

    assert_eq!(map["dragonland"], 20);
}

