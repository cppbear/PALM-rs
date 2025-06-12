// Answer 0

#[test]
fn test_new_map() {
    let map: indexmap::IndexMap<(), ()> = indexmap::IndexMap::new();
    assert!(map.is_empty());
}

