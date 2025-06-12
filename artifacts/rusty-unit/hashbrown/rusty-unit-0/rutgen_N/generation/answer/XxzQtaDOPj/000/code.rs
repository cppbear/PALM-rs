// Answer 0

#[test]
fn test_or_insert_nonexistent_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(3);
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_existing_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(3);
    *map.entry("poneyland").or_insert(10) *= 2;
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_default_value() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    assert_eq!(map.entry("unicorn").or_insert(42), &mut 42);
    assert_eq!(map["unicorn"], 42);
}

#[test]
fn test_or_insert_with_empty_key() {
    use hashbrown::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let key = "";
    *map.entry(key).or_insert(1) += 1;
    assert_eq!(map[key], 2);
}

