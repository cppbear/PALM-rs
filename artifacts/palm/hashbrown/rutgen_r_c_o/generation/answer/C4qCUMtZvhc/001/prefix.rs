// Answer 0

#[test]
fn test_or_default_vacant_entry_string_option() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    let key: &str = "poneyland";

    let value = map.entry_ref(key).or_default();
}

#[test]
fn test_or_default_vacant_entry_u32_i32() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHashBuilder;

    let mut map: HashMap<u32, i32> = HashMap::new();
    let key: &u32 = &42;

    let value = map.entry_ref(key).or_default();
}

#[test]
fn test_or_default_vacant_entry_string_f64() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<String, f64> = HashMap::new();
    let key: &String = &"horseland".to_string();

    let value = map.entry_ref(key).or_default();
}

