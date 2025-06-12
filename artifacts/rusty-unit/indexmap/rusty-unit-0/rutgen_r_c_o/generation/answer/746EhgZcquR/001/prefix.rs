// Answer 0

#[test]
fn test_swap_remove_small_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_index(0).unwrap().clone(),
        hash_builder: PhantomData,
    };
    entry.swap_remove();
}

#[test]
fn test_swap_remove_edge_case_empty() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_index(0).unwrap().clone(),
        hash_builder: PhantomData,
    };
    // This test is expected to panic because there are no entries to remove
    let result = std::panic::catch_unwind(|| {
        entry.swap_remove();
    });
    assert!(result.is_err());
}

#[test]
fn test_swap_remove_mid_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_index(1).unwrap().clone(),
        hash_builder: PhantomData,
    };
    entry.swap_remove();
}

#[test]
fn test_swap_remove_last_element() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "one".to_string());
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_index(0).unwrap().clone(),
        hash_builder: PhantomData,
    };
    entry.swap_remove();
}

#[test]
fn test_swap_remove_large_map() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    for i in 0..1000 {
        map.insert(i, format!("value_{}", i));
    }
    let entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_index(500).unwrap().clone(),
        hash_builder: PhantomData,
    };
    entry.swap_remove();
}

