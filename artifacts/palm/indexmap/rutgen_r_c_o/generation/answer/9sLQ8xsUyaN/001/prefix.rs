// Answer 0

#[test]
fn test_remove_entry_non_empty() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    let mut entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_entry(&1).unwrap().occupied_entry(),
        hash_builder: PhantomData,
    };
    
    let (key, value) = entry.remove_entry();
}

#[test]
fn test_remove_entry_last_element() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    let mut entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_entry(&1).unwrap().occupied_entry(),
        hash_builder: PhantomData,
    };
    
    let (key, value) = entry.remove_entry();
}

#[test]
#[should_panic]
fn test_remove_entry_empty() {
    let mut map: IndexMap<usize, &str> = IndexMap::new();
    let mut entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: panic!("No occupied entry to remove"), // This will panic as the map is empty
        hash_builder: PhantomData,
    };
    
    let (key, value) = entry.remove_entry();
}

#[test]
fn test_remove_entry_specific_index() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    
    let mut entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_entry(&2).unwrap().occupied_entry(),
        hash_builder: PhantomData,
    };
    
    let (key, value) = entry.remove_entry();
}

#[test]
fn test_remove_entry_after_multiple_insertions() {
    let mut map = IndexMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    
    let mut entry = RawOccupiedEntryMut {
        entries: &mut map,
        index: map.get_entry(&3).unwrap().occupied_entry(),
        hash_builder: PhantomData,
    };
    
    let (key, value) = entry.remove_entry();
}

