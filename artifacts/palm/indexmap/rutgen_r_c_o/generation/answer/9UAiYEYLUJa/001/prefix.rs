// Answer 0

#[test]
fn test_swap_indices_valid_bounds() {
    let mut entries = vec![1, 2, 3, 4, 5];
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0));
    occupied_entry.swap_indices(1);
}

#[test]
fn test_swap_indices_edge_case_first_last() {
    let mut entries = vec![10, 20, 30, 40];
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0));
    occupied_entry.swap_indices(3);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_high() {
    let mut entries = vec![5, 15, 25];
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(1));
    occupied_entry.swap_indices(3);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_low() {
    let mut entries = vec![7, 14, 21];
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(2));
    occupied_entry.swap_indices(0);
}

#[test]
fn test_swap_indices_same_index() {
    let mut entries = vec![100, 200];
    let mut occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(0));
    occupied_entry.swap_indices(0);
}

