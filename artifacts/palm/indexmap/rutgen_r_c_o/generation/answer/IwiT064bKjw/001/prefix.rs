// Answer 0

#[test]
fn test_pop_empty_entries() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = map.pop();
}

#[test]
fn test_pop_empty_entries_with_capacity() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(10);
    let result = map.pop();
}

