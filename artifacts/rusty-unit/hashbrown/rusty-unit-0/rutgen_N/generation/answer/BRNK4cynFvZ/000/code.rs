// Answer 0

#[test]
fn test_or_insert_with_key_vacant() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    
    map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    assert_eq!(map["poneyland"], 9);
}

#[test]
fn test_or_insert_with_key_occupied() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    
    map.entry("poneyland").or_insert_with_key(|key| key.chars().count());
    *map.entry("poneyland").or_insert_with_key(|key| key.chars().count() * 10) *= 2;
    assert_eq!(map["poneyland"], 18);
}

#[test]
fn test_or_insert_with_key_with_different_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    
    map.entry("dragonland").or_insert_with_key(|key| key.len() * 2);
    assert_eq!(map["dragonland"], 20); // "dragonland" has 10 characters, so 10 * 2
}

#[test]
fn test_or_insert_with_key_empty_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, usize> = HashMap::new();
    
    map.entry("").or_insert_with_key(|key| key.len());
    assert_eq!(map[""], 0); // empty key should have length 0
}

