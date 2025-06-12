// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    use crate::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = map.entry("poneyland");
    let value = entry.or_insert_with(|| 3);
    assert_eq!(*value, 3);
    assert_eq!(map["poneyland"], 3);
}

#[test]
fn test_or_insert_with_occupied_entry() {
    use crate::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 3);
    let entry = map.entry("poneyland");
    let value = entry.or_insert_with(|| 10);
    *value *= 2;
    assert_eq!(*value, 6);
    assert_eq!(map["poneyland"], 6);
}

#[test]
fn test_or_insert_with_multiple_keys() {
    use crate::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry1 = map.entry("poneyland");
    let value1 = entry1.or_insert_with(|| 5);
    assert_eq!(*value1, 5);
    
    let entry2 = map.entry("unicornland");
    let value2 = entry2.or_insert_with(|| 10);
    assert_eq!(*value2, 10);

    assert_eq!(map["poneyland"], 5);
    assert_eq!(map["unicornland"], 10);
}

#[test]
fn test_or_insert_with_chained_entries() {
    use crate::HashMap;

    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert_with(|| 2);
    map.entry("poneyland").or_insert_with(|| 4);
    
    assert_eq!(map["poneyland"], 2);
}

