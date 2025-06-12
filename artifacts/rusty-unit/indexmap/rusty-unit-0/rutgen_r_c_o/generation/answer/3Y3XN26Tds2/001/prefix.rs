// Answer 0

#[test]
fn test_sort_by_cached_key_small() {
    let mut map = IndexMap::new();
    map.insert("apple", 3);
    map.insert("banana", 1);
    map.insert("cherry", 2);
    map.sort_by_cached_key(|_k, v| *v);
}

#[test]
fn test_sort_by_cached_key_empty() {
    let mut map: IndexMap<String, i32> = IndexMap::new();
    map.sort_by_cached_key(|_k, _v| 0);
}

#[test]
fn test_sort_by_cached_key_large() {
    let mut map = IndexMap::new();
    for i in 0..1_000_000 {
        map.insert(i.to_string(), 100 - (i % 100));
    }
    map.sort_by_cached_key(|_k, v| *v);
}

#[test]
fn test_sort_by_cached_key_reverse_order() {
    let mut map = IndexMap::new();
    map.insert("z", 1);
    map.insert("y", 2);
    map.insert("x", 3);
    map.sort_by_cached_key(|_k, v| *v);
}

#[test]
fn test_sort_by_cached_key_identical_keys() {
    let mut map = IndexMap::new();
    map.insert("same", 1);
    map.insert("same", 2);
    map.insert("same", 3);
    map.sort_by_cached_key(|_k, v| *v);
}

#[test]
fn test_sort_by_cached_key_sorting_same_values() {
    let mut map = IndexMap::new();
    map.insert("b", 1);
    map.insert("a", 1);
    map.insert("c", 1);
    map.sort_by_cached_key(|k, _v| k);
}

