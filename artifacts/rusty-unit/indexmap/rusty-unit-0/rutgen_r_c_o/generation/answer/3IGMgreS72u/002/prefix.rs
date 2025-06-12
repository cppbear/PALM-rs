// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());
    
    let entry: RawEntryMut<i32, String, _> = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.get_index(1).unwrap(),
        hash_builder: PhantomData,
    });
    
    entry.and_modify(|k, v| {
        *k += 1;
        v.push_str("_modified");
    });
}

#[test]
fn test_and_modify_with_multiple_modifications() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(3, "value3".to_string());
    
    let entry: RawEntryMut<i32, String, _> = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.get_index(3).unwrap(),
        hash_builder: PhantomData,
    });
    
    entry.and_modify(|k, v| {
        *k *= 2;
        v.clear();
        v.push_str("modified_again");
    });
}

#[test]
fn test_and_modify_with_edge_case() {
    let mut map: IndexMap<i32, String> = IndexMap::new();
    map.insert(4, "value4".to_string());
    
    let entry: RawEntryMut<i32, String, _> = RawEntryMut::Occupied(RawOccupiedEntryMut {
        entries: &mut map.entries,
        index: map.get_index(4).unwrap(),
        hash_builder: PhantomData,
    });
    
    entry.and_modify(|k, v| {
        *k = 0;
        v.push_str(" - reset");
    });
}

