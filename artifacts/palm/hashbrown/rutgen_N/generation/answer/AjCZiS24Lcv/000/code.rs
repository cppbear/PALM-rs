// Answer 0

#[test]
fn test_hashmap_new() {
    use hashbrown::HashMap;

    let map: HashMap<&str, i32> = HashMap::new();
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

#[test]
fn test_hashmap_with_hasher() {
    use hashbrown::HashMap;
    use std::collections::hash_map::RandomState;

    let map: HashMap<&str, i32> = HashMap::with_hasher(RandomState::new());
    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);
}

