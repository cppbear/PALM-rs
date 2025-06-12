// Answer 0

#[test]
fn test_swap_remove_entry_existing_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    map.insert(3, String::from("three"));
    
    let result = map.swap_remove_entry(&2);
}

#[test]
fn test_swap_remove_entry_multiple_keys() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    for i in 1..=5 {
        map.insert(i, format!("value {}", i));
    }
    
    let result = map.swap_remove_entry(&3);
}

#[test]
fn test_swap_remove_entry_first_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(10, String::from("ten"));
    map.insert(20, String::from("twenty"));
    
    let result = map.swap_remove_entry(&10);
}

#[test]
fn test_swap_remove_entry_last_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(1000, String::from("one thousand"));
    map.insert(999, String::from("nine hundred ninety-nine"));
    
    let result = map.swap_remove_entry(&1000);
}

#[test]
fn test_swap_remove_entry_non_existent_key() {
    let mut map: IndexMap<i32, String, RandomState> = IndexMap::new();
    map.insert(500, String::from("five hundred"));
    
    let result = map.swap_remove_entry(&1000);
}

