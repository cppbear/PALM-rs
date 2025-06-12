// Answer 0

#[derive(Debug)]
struct DummyEntry {
    key: i32,
    value: String,
}

struct DummyMap {
    entries: Vec<DummyEntry>,
}

impl DummyMap {
    fn as_entries(&self) -> &[DummyEntry] {
        &self.entries
    }
}

struct DummyStruct {
    map: DummyMap,
}

impl DummyStruct {
    fn new(entries: Vec<DummyEntry>) -> Self {
        Self {
            map: DummyMap { entries },
        }
    }

    fn as_entries(&self) -> &[DummyEntry] {
        self.map.as_entries()
    }
}

#[test]
fn test_as_entries_empty() {
    let struct_instance = DummyStruct::new(vec![]);
    let entries = struct_instance.as_entries();
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_as_entries_single() {
    let struct_instance = DummyStruct::new(vec![DummyEntry { key: 1, value: "one".to_string() }]);
    let entries = struct_instance.as_entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "one");
}

#[test]
fn test_as_entries_multiple() {
    let entries = vec![
        DummyEntry { key: 1, value: "one".to_string() },
        DummyEntry { key: 2, value: "two".to_string() },
        DummyEntry { key: 3, value: "three".to_string() },
    ];
    let struct_instance = DummyStruct::new(entries);
    let result_entries = struct_instance.as_entries();
    assert_eq!(result_entries.len(), 3);
    assert_eq!(result_entries[0].key, 1);
    assert_eq!(result_entries[1].key, 2);
    assert_eq!(result_entries[2].key, 3);
}

#[test]
fn test_as_entries_large() {
    let entries: Vec<DummyEntry> = (0..1000)
        .map(|i| DummyEntry { key: i, value: i.to_string() })
        .collect();
    let struct_instance = DummyStruct::new(entries);
    let result_entries = struct_instance.as_entries();
    assert_eq!(result_entries.len(), 1000);
    assert_eq!(result_entries[999].key, 999);
    assert_eq!(result_entries[999].value, "999");
}

