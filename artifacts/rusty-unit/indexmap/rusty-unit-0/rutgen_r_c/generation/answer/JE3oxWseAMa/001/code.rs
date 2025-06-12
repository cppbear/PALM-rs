// Answer 0

#[test]
fn test_indexed_entry_debug_fmt() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries {
        data: vec![(TestKey, TestValue)],
    };
    let mut indices = Indices::new();
    let mut map = IndexMapCore {
        entries: &mut entries,
        indices: &mut indices,
    };

    let entry = IndexedEntry::new(&mut map, 0);

    let result = format!("{:?}", entry);
    assert!(result.contains("IndexedEntry"));
    assert!(result.contains("key"));
    assert!(result.contains("value"));
    assert!(result.contains("index"));
} 

#[test]
#[should_panic]
fn test_indexed_entry_debug_fmt_out_of_bounds() {
    struct TestKey;
    struct TestValue;

    let mut entries = Entries {
        data: vec![(TestKey, TestValue)],
    };
    let mut indices = Indices::new();
    let mut map = IndexMapCore {
        entries: &mut entries,
        indices: &mut indices,
    };

    let entry = IndexedEntry::new(&mut map, 1); // Out of bounds index

    let _ = format!("{:?}", entry); // This should panic
} 

#[test]
fn test_indexed_entry_debug_fmt_empty() {
    struct TestKey;
    struct TestValue;

    let mut empty_entries = Entries {
        data: Vec::new(),
    };
    let mut indices = Indices::new();
    let mut map = IndexMapCore {
        entries: &mut empty_entries,
        indices: &mut indices,
    };

    let entry = IndexedEntry::new(&mut map, 0); // This is still valid for testing

    let result = format!("{:?}", entry);
    assert!(result.contains("IndexedEntry"));
    assert!(result.contains("key"));
    assert!(result.contains("value"));
    assert!(result.contains("index"));
} 

