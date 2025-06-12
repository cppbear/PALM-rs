// Answer 0

#[test]
fn test_or_default_vacant_entry() {
    use crate::HashMap;
    
    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    
    // Testing or_default on a nonexistent key
    let value_ref = map.entry_ref("poneyland").or_default();
    assert_eq!(map["poneyland"], None);
    assert_eq!(*value_ref, None);
}

#[test]
fn test_or_default_occupied_entry() {
    use crate::HashMap;
    
    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    
    // Inserting an existing key
    map.insert("horseland".to_string(), Some(3));
    
    // Testing or_default on an existing key
    let value_ref = map.entry_ref("horseland").or_default();
    assert_eq!(map["horseland"], Some(3));
    assert_eq!(*value_ref, Some(3));
}

#[test]
fn test_or_default_multiple_keys() {
    use crate::HashMap;
    
    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    
    // Using or_default on multiple entries
    let value1_ref = map.entry_ref("key1").or_default();
    assert_eq!(map["key1"], None);
    
    let value2_ref = map.entry_ref("key2").or_default();
    assert_eq!(map["key2"], None);
    
    // Ensure that both values are now the default value (None)
    assert_eq!(*value1_ref, None);
    assert_eq!(*value2_ref, None);
}

