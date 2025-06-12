// Answer 0

#[test]
fn test_move_index_within_bounds() {
    let mut entries = vec![1, 2, 3, 4, 5];
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(0), // Initial index 0
        hash_builder: PhantomData,
    };
    raw_entry.move_index(2); // Moving index 0 to 2
}

#[test]
fn test_move_index_to_start() {
    let mut entries = vec![1, 2, 3, 4, 5];
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(4), // Initial index 4
        hash_builder: PhantomData,
    };
    raw_entry.move_index(0); // Moving index 4 to 0
}

#[test]
fn test_move_index_to_end() {
    let mut entries = vec![1, 2, 3, 4, 5];
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(1), // Initial index 1
        hash_builder: PhantomData,
    };
    raw_entry.move_index(4); // Moving index 1 to 4
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_negative() {
    let mut entries = vec![1, 2, 3, 4, 5];
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(2), // Initial index 2
        hash_builder: PhantomData,
    };
    raw_entry.move_index(-1); // Attempting to move to an invalid index
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_high() {
    let mut entries = vec![1, 2, 3, 4, 5];
    let raw_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(2), // Initial index 2
        hash_builder: PhantomData,
    };
    raw_entry.move_index(6); // Attempting to move to an out of bounds index
}

