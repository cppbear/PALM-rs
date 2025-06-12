// Answer 0

#[test]
fn test_swap_indices_valid() {
    let mut entries: Vec<(i32, i32)> = vec![(0, 0), (1, 1), (2, 2)];
    let occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    occupied_entry_mut.swap_indices(1);
}

#[test]
fn test_swap_indices_valid_edge() {
    let mut entries: Vec<(i32, i32)> = vec![(0, 0), (1, 1), (2, 2)];
    let occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    occupied_entry_mut.swap_indices(2);
}

#[should_panic]
fn test_swap_indices_out_of_bounds_high() {
    let mut entries: Vec<(i32, i32)> = vec![(0, 0)];
    let occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    occupied_entry_mut.swap_indices(1);
}

#[should_panic]
fn test_swap_indices_out_of_bounds_low() {
    let mut entries: Vec<(i32, i32)> = vec![(0, 0)];
    let occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    occupied_entry_mut.swap_indices(usize::MAX);
}

#[should_panic]
fn test_swap_indices_identical() {
    let mut entries: Vec<(i32, i32)> = vec![(0, 0), (1, 1)];
    let occupied_entry_mut = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::new(0),
        hash_builder: PhantomData,
    };
    occupied_entry_mut.swap_indices(0);
}

