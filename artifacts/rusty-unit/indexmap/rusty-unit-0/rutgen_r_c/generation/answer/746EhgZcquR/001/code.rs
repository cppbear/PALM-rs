// Answer 0

#[test]
fn test_swap_remove_non_empty_map() {
    use crate::IndexMap;

    // Create a new IndexMap and populate it with some key-value pairs.
    let mut map: IndexMap<usize, &str> = IndexMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    // Get a mutable reference to an entry using the key 2.
    let entry = map.entry(2).or_insert("temp");
    let raw_entry_mut = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::from_entry(entry),
        hash_builder: PhantomData,
    };

    // Perform the swap remove operation.
    let removed_value = raw_entry_mut.swap_remove();

    // Verify that the removed value is correct.
    assert_eq!(removed_value, "two");
    assert!(!map.contains_key(&2)); // Ensure the key has been removed.
}

#[test]
#[should_panic(expected = "attempt to remove entry that does not exist")]
fn test_swap_remove_empty_map() {
    use crate::IndexMap;

    // Create an empty IndexMap.
    let mut map: IndexMap<usize, &str> = IndexMap::new();

    // Attempt to create an entry that doesn't exist will cause a panic.
    let entry = map.entry(4).or_insert("temp");
    let raw_entry_mut = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::from_entry(entry),
        hash_builder: PhantomData,
    };

    // Attempting to swap remove on an empty map should panic.
    let _removed_value = raw_entry_mut.swap_remove();
}

#[test]
fn test_swap_remove_with_single_entry() {
    use crate::IndexMap;

    // Create an IndexMap with a single key-value pair.
    let mut map: IndexMap<usize, &str> = IndexMap::new();
    map.insert(1, "only_one");

    // Get a mutable reference to the entry.
    let entry = map.entry(1).or_insert("temp");
    let raw_entry_mut = RawOccupiedEntryMut {
        entries: &mut map,
        index: hash_table::OccupiedEntry::from_entry(entry),
        hash_builder: PhantomData,
    };

    // Perform the swap remove operation.
    let removed_value = raw_entry_mut.swap_remove();

    // Verify that the removed value is correct and the map is empty now.
    assert_eq!(removed_value, "only_one");
    assert!(map.is_empty()); // Ensure the map is now empty.
}

