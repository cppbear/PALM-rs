// Answer 0

#[test]
fn test_or_insert_occupied_entry() {
    let mut map = IndexMap::new();
    map.insert(1, 10);
    let mut entries = Entries::from_index_map(&mut map);
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(1).unwrap(),
        hash_builder: PhantomData,
    };
    let entry = RawEntryMut::Occupied(occupied_entry);
    let _ = entry.or_insert(2, 20);
}

#[test]
fn test_or_insert_with_another_occupied_entry() {
    let mut map = IndexMap::new();
    map.insert(5, 50);
    let mut entries = Entries::from_index_map(&mut map);
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(5).unwrap(),
        hash_builder: PhantomData,
    };
    let entry = RawEntryMut::Occupied(occupied_entry);
    let _ = entry.or_insert(3, 30);
}

#[test]
fn test_or_insert_same_key_occupied_entry() {
    let mut map = IndexMap::new();
    map.insert(7, 70);
    let mut entries = Entries::from_index_map(&mut map);
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(7).unwrap(),
        hash_builder: PhantomData,
    };
    let entry = RawEntryMut::Occupied(occupied_entry);
    let _ = entry.or_insert(7, 75);
}

#[test]
fn test_or_insert_large_value_occupied_entry() {
    let mut map = IndexMap::new();
    map.insert(10, 100);
    let mut entries = Entries::from_index_map(&mut map);
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(10).unwrap(),
        hash_builder: PhantomData,
    };
    let entry = RawEntryMut::Occupied(occupied_entry);
    let _ = entry.or_insert(15, 150);
}

#[test]
fn test_or_insert_boundary_condition_for_occupied() {
    let mut map = IndexMap::new();
    map.insert(1000, 1000);
    let mut entries = Entries::from_index_map(&mut map);
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(1000).unwrap(),
        hash_builder: PhantomData,
    };
    let entry = RawEntryMut::Occupied(occupied_entry);
    let _ = entry.or_insert(999, 999);
}

