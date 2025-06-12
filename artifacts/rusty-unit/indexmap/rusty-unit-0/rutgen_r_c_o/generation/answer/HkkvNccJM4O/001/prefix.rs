// Answer 0

#[test]
fn test_move_index_valid_range_up() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 2);
    entry.move_index(4);
}

#[test]
fn test_move_index_valid_range_down() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 4);
    entry.move_index(2);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_high() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 0);
    entry.move_index(UPPER_BOUND + 1);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_low() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, UPPER_BOUND);
    entry.move_index((0 as isize) - 1);
}

#[test]
fn test_move_index_on_boundaries() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry_start = IndexedEntry::new(&mut map, 0);
    let entry_end = IndexedEntry::new(&mut map, UPPER_BOUND);
    entry_start.move_index(1);
    entry_end.move_index(UPPER_BOUND - 1);
}

#[test]
#[should_panic]
fn test_move_index_invalid_both_bounds() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let mut map = IndexMapCore::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, UPPER_BOUND);
    entry.move_index(UPPER_BOUND + 1);
}

