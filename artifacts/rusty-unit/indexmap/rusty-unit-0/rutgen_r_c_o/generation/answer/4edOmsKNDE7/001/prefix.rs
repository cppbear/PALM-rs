// Answer 0

#[test]
fn test_shift_remove_single_entry() {
    let mut entries = IndexMap::new();
    entries.insert(1, 100);
    let index = entries.get_index_of(&1).unwrap();
    let occupied = RawOccupiedEntryMut { entries: &mut entries, index, hash_builder: PhantomData };
    let _ = occupied.shift_remove();
}

#[test]
fn test_shift_remove_multiple_entries() {
    let mut entries = IndexMap::new();
    entries.insert(1, 100);
    entries.insert(2, 200);
    entries.insert(3, 300);
    let index = entries.get_index_of(&2).unwrap();
    let occupied = RawOccupiedEntryMut { entries: &mut entries, index, hash_builder: PhantomData };
    let _ = occupied.shift_remove();
}

#[test]
fn test_shift_remove_first_entry() {
    let mut entries = IndexMap::new();
    entries.insert(1, 100);
    entries.insert(2, 200);
    entries.insert(3, 300);
    let index = entries.get_index_of(&1).unwrap();
    let occupied = RawOccupiedEntryMut { entries: &mut entries, index, hash_builder: PhantomData };
    let _ = occupied.shift_remove();
}

#[test]
fn test_shift_remove_last_entry() {
    let mut entries = IndexMap::new();
    entries.insert(1, 100);
    entries.insert(2, 200);
    entries.insert(3, 300);
    let index = entries.get_index_of(&3).unwrap();
    let occupied = RawOccupiedEntryMut { entries: &mut entries, index, hash_builder: PhantomData };
    let _ = occupied.shift_remove();
}

#[test]
fn test_shift_remove_with_large_map() {
    let mut entries = IndexMap::new();
    for i in 1..=1000 {
        entries.insert(i, i * 10);
    }
    let index = entries.get_index_of(&500).unwrap();
    let occupied = RawOccupiedEntryMut { entries: &mut entries, index, hash_builder: PhantomData };
    let _ = occupied.shift_remove();
}

