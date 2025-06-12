// Answer 0

#[test]
fn test_remove_existing_key() {
    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let _ = map.remove(&2);
}

#[test]
fn test_remove_non_existing_key() {
    let mut map = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let _ = map.remove(&4);
}

#[test]
fn test_remove_edge_case_empty_map() {
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    let _ = map.remove(&1);
}

#[test]
fn test_remove_multiple_keys() {
    let mut map = IndexMap::new();
    for i in 1..=5 {
        map.insert(i, i * 10);
    }
    let _ = map.remove(&3);
    let _ = map.remove(&1);
}

#[test]
fn test_remove_with_high_key_value() {
    let mut map = IndexMap::new();
    for i in 0..100_000 {
        map.insert(i, i as i32 * 10);
    }
    let _ = map.remove(&99999);
}

#[test]
fn test_remove_key_from_large_map() {
    let mut map = IndexMap::new();
    for i in (1..=100_000).rev() {
        map.insert(i, i * 10);
    }
    let _ = map.remove(&100_000);
}

#[test]
fn test_remove_with_non_hashable_key() {
    struct NonHashable;
    let mut map = IndexMap::new();
    map.insert(NonHashable, 1);
    let key = NonHashable;
    let _ = map.remove(&key);
}

