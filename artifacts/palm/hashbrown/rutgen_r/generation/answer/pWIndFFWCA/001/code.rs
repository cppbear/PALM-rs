// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;
    use std::hash::BuildHasherDefault;

    struct MyDefault;

    impl Default for MyDefault {
        fn default() -> Self {
            MyDefault
        }
    }

    let mut map: HashMap<String, MyDefault, BuildHasherDefault<std::collections::hash_map::RandomState>> = HashMap::new();
    
    let value = map.entry_ref("key1").or_insert(MyDefault::default());
    
    // Verifying that the value was inserted and is the default instance
    assert_eq!(value, &MyDefault::default());
}

#[test]
fn test_or_insert_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;
    use std::hash::BuildHasherDefault;

    struct MyValue(u32);

    impl Default for MyValue {
        fn default() -> Self {
            MyValue(0)
        }
    }

    let mut map: HashMap<String, MyValue, BuildHasherDefault<std::collections::hash_map::RandomState>> = HashMap::new();
    map.insert("key2".to_string(), MyValue(5));
    
    {
        let value = map.entry_ref("key2").or_insert(MyValue(10));
        value.0 *= 2;  // Mutating the existing entry
    }
    
    // Verifying that the value has been updated correctly
    assert_eq!(map["key2"].0, 10);
}

