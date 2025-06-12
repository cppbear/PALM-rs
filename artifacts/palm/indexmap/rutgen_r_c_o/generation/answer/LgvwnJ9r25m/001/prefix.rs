// Answer 0

#[test]
fn test_shift_remove_valid_entry() {
    let mut entries = IndexMapCore::new();
    entries.insert(1, "Value1");
    entries.insert(2, "Value2");
    entries.insert(3, "Value3");

    let occupied_entry = OccupiedEntry::new(&mut entries, entries.get_occupied_entry(&2).unwrap());
    let value = occupied_entry.shift_remove();
}

#[test]
fn test_shift_remove_first_entry() {
    let mut entries = IndexMapCore::new();
    entries.insert(1, "Value1");
    entries.insert(2, "Value2");
    entries.insert(3, "Value3");

    let occupied_entry = OccupiedEntry::new(&mut entries, entries.get_occupied_entry(&1).unwrap());
    let value = occupied_entry.shift_remove();
}

#[test]
fn test_shift_remove_last_entry() {
    let mut entries = IndexMapCore::new();
    entries.insert(1, "Value1");
    entries.insert(2, "Value2");
    entries.insert(3, "Value3");

    let occupied_entry = OccupiedEntry::new(&mut entries, entries.get_occupied_entry(&3).unwrap());
    let value = occupied_entry.shift_remove();
}

#[test]
#[should_panic]
fn test_shift_remove_empty_entries() {
    let mut entries = IndexMapCore::new();

    let occupied_entry = OccupiedEntry::new(&mut entries, entries.get_occupied_entry(&1).unwrap());
    let value = occupied_entry.shift_remove();
}

#[test]
fn test_shift_remove_multiple_identical_keys() {
    let mut entries = IndexMapCore::new();
    entries.insert(1, "Value1");
    entries.insert(1, "Value2");
    entries.insert(1, "Value3");

    let occupied_entry = OccupiedEntry::new(&mut entries, entries.get_occupied_entry(&1).unwrap());
    let value = occupied_entry.shift_remove();
}

