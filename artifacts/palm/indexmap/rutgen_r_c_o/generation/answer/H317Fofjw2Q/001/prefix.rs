// Answer 0

#[test]
fn test_swap_remove_entry_valid() {
    let mut entries = IndexMap::new();
    for i in 1..=10 {
        entries.insert(i, i * 10);
    }
    let index = 5; // Valid index within the bounds of existing entries
    let mut entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(index).unwrap(),
        hash_builder: PhantomData,
    };
    entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_boundaries() {
    let mut entries = IndexMap::new();
    for i in 1..=20 {
        entries.insert(i, i * 10);
    }
    let last_index = entries.len() - 1;
    let mut entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(last_index).unwrap(),
        hash_builder: PhantomData,
    };
    entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_empty() {
    let mut entries: IndexMap<i32, i32> = IndexMap::new();
    assert!(entries.is_empty()); // Ensure it's empty for context
    if let Some(entry) = entries.occupied_entry(0).ok() {
        let mut entry = RawOccupiedEntryMut {
            entries: &mut entries,
            index: entry,
            hash_builder: PhantomData,
        };
        entry.swap_remove_entry(); // Should not panic, but there will be no effect
    }
}

#[test]
fn test_swap_remove_entry_start() {
    let mut entries = IndexMap::new();
    for i in 1..=15 {
        entries.insert(i, i * 100);
    }
    let first_index = 0; // First element index
    let mut entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(first_index).unwrap(),
        hash_builder: PhantomData,
    };
    entry.swap_remove_entry();
}

#[test]
fn test_swap_remove_entry_large_data() {
    let mut entries = IndexMap::new();
    for i in 1..=1000 {
        entries.insert(i, i * 1000);
    }
    let random_index = 500; // Middle of the entries
    let mut entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: entries.occupied_entry(random_index).unwrap(),
        hash_builder: PhantomData,
    };
    entry.swap_remove_entry();
}

