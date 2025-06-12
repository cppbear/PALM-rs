// Answer 0

#[test]
fn test_index_zero() {
    let mut map = IndexMapCore::new();
    let indexed_entry = IndexedEntry::new(&mut map, 0);
    indexed_entry.index();
}

#[test]
fn test_index_max() {
    let mut map = IndexMapCore::new();
    let indexed_entry = IndexedEntry::new(&mut map, usize::MAX);
    indexed_entry.index();
}

#[test]
fn test_index_middle() {
    let mut map = IndexMapCore::new();
    let indexed_entry = IndexedEntry::new(&mut map, 10);
    indexed_entry.index();
}

#[test]
fn test_index_large_value() {
    let mut map = IndexMapCore::new();
    let indexed_entry = IndexedEntry::new(&mut map, 1000);
    indexed_entry.index();
}

