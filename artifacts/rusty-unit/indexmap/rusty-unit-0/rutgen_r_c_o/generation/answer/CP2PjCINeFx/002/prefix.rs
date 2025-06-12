// Answer 0

#[test]
fn test_truncate_exact_length_zero() {
    let mut map = IndexMapCore::new();
    map.truncate(0);
}

#[test]
fn test_truncate_exact_length_one() {
    let mut map = IndexMapCore::with_capacity(1);
    map.truncate(1);
}

#[test]
fn test_truncate_exact_length_max_capacity() {
    let mut map = IndexMapCore::with_capacity(IndexMapCore::MAX_ENTRIES_CAPACITY);
    map.truncate(IndexMapCore::MAX_ENTRIES_CAPACITY);
}

#[test]
fn test_truncate_exact_length_after_insertion() {
    let mut map = IndexMapCore::new();
    let key = 1usize;
    let value = "value1";
    map.push_entry(0, key, value);
    map.truncate(1);
}

#[test]
#[should_panic]
fn test_truncate_beyond_current_length() {
    let mut map = IndexMapCore::new();
    map.truncate(1);
}

