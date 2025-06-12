// Answer 0

#[test]
fn test_move_index_within_bounds_upwards() {
    let mut indices = vec![0, 1, 2, 3, 4];
    let mut entries = vec![1, 2, 3, 4, 5];
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut indices, 2));
    occupied_entry.move_index(4);
}

#[test]
fn test_move_index_within_bounds_downwards() {
    let mut indices = vec![0, 1, 2, 3, 4];
    let mut entries = vec![1, 2, 3, 4, 5];
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut indices, 4));
    occupied_entry.move_index(1);
}

#[test]
#[should_panic]
fn test_move_index_below_bounds() {
    let mut indices = vec![0, 1, 2, 3, 4];
    let mut entries = vec![1, 2, 3, 4, 5];
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut indices, 2));
    occupied_entry.move_index(5);
}

#[test]
#[should_panic]
fn test_move_index_above_bounds() {
    let mut indices = vec![0, 1, 2, 3, 4];
    let mut entries = vec![1, 2, 3, 4, 5];
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut indices, 0));
    occupied_entry.move_index(-1);
}

#[test]
fn test_move_index_to_same_index() {
    let mut indices = vec![0, 1, 2, 3, 4];
    let mut entries = vec![1, 2, 3, 4, 5];
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut indices, 3));
    occupied_entry.move_index(3);
}

