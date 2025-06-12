// Answer 0

#[test]
fn test_shift_remove_first_element() {
    let mut map = IndexMapCore::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = entry.shift_remove();
}

#[test]
fn test_shift_remove_middle_element() {
    let mut map = IndexMapCore::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let entry = IndexedEntry::new(&mut map, 1);
    let _ = entry.shift_remove();
}

#[test]
fn test_shift_remove_last_element() {
    let mut map = IndexMapCore::new();
    map.insert("a", 1);
    map.insert("b", 2);
    let entry = IndexedEntry::new(&mut map, 1);
    let _ = entry.shift_remove();
}

#[test]
fn test_shift_remove_single_element() {
    let mut map = IndexMapCore::new();
    map.insert("a", 1);
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = entry.shift_remove();
}

#[should_panic]
fn test_shift_remove_empty_map() {
    let mut map = IndexMapCore::new();
    let entry = IndexedEntry::new(&mut map, 0);
    let _ = entry.shift_remove();
}

#[test]
fn test_shift_remove_after_multiple_removals() {
    let mut map = IndexMapCore::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let entry1 = IndexedEntry::new(&mut map, 1);
    let _ = entry1.shift_remove();
    let entry2 = IndexedEntry::new(&mut map, 1);
    let _ = entry2.shift_remove(); // should now remove "c"
}

