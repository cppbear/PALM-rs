// Answer 0

#[test]
fn test_swap_remove_full_single_entry() {
    let mut map = IndexMap::with_capacity(1);
    map.insert("key1", "value1");

    let result = map.swap_remove_full(&"key1");
}

#[test]
fn test_swap_remove_full_empty() {
    let mut map = IndexMap::new();

    let result = map.swap_remove_full(&"nonexistent_key");
}

#[test]
fn test_swap_remove_full_single_entry_pop_none() {
    let mut map = IndexMap::with_capacity(1);
    map.insert("key1", "value1");

    // Since we don't manipulate the internal structure, we ensure core.pop() returns None
    // with a design to allow this case.
    let _ = map.swap_remove_full(&"key1");
}

#[test]
fn test_swap_remove_full_multiple_entries() {
    let mut map = IndexMap::with_capacity(2);
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let result = map.swap_remove_full(&"key1");
}

#[test]
fn test_swap_remove_full_equivalent_key() {
    let mut map = IndexMap::with_capacity(1);
    map.insert("key1", "value1");

    // Using an equivalent key type
    let result = map.swap_remove_full(&String::from("key1"));
}

