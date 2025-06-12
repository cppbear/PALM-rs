// Answer 0

#[test]
fn test_get_full_mut_existing_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    let key = 2;
    let result = map.get_full_mut(&key);
}

#[test]
fn test_get_full_mut_existing_key_first_entry() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let key = 0;
    let result = map.get_full_mut(&key);
}

#[test]
fn test_get_full_mut_existing_key_last_entry() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let key = 2;
    let result = map.get_full_mut(&key);
}

#[test]
fn test_get_full_mut_existing_key_multiple_identical_entries() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, "value".to_string());
    map.insert(1, "value2".to_string());

    let key = 1;
    let result = map.get_full_mut(&key);
}

#[test]
fn test_get_full_mut_key_at_capacity() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    for i in 0..100 {
        map.insert(i, format!("value{}", i));
    }

    let key = 99;
    let result = map.get_full_mut(&key);
}

