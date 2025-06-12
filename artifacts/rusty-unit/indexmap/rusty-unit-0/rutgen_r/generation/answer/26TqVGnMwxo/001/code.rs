// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct Table<K, V> {
    entries: Vec<Entry<K, V>>,
}

struct RefMut<'a, K, V> {
    table: &'a mut Table<K, V>,
    entries: &'a mut Vec<Entry<K, V>>,
}

impl<'a, K, V> RefMut<'a, K, V> {
    fn new(table: &'a mut Table<K, V>, entries: &'a mut Vec<Entry<K, V>>) -> Self {
        Self { table, entries }
    }
}

struct RawEntry<K, V> {
    index: Table<K, V>,
    entries: Vec<Entry<K, V>>,
}

impl<K, V> RawEntry<K, V> {
    fn into_ref_mut(self) -> RefMut<K, V> {
        RefMut::new(&mut self.index, &mut self.entries)
    }
}

#[test]
fn test_into_ref_mut_with_non_empty_entries() {
    let mut table = Table {
        entries: vec![Entry { key: 1, value: "a" }],
    };
    let entries = vec![Entry { key: 1, value: "a" }];
    let raw_entry = RawEntry { index: table, entries };

    let ref_mut = raw_entry.into_ref_mut();
    assert_eq!(ref_mut.table.entries.len(), 1);
    assert_eq!(ref_mut.entries.len(), 1);
}

#[test]
fn test_into_ref_mut_with_empty_entries() {
    let mut table = Table {
        entries: Vec::new(),
    };
    let entries = Vec::new();
    let raw_entry = RawEntry { index: table, entries };

    let ref_mut = raw_entry.into_ref_mut();
    assert_eq!(ref_mut.table.entries.len(), 0);
    assert_eq!(ref_mut.entries.len(), 0);
}

#[should_panic(expected = "attempt to modify borrowed content")]
#[test]
fn test_into_ref_mut_panic_conditions() {
    let mut table = Table {
        entries: vec![Entry { key: 2, value: "b" }],
    };
    let entries = vec![Entry { key: 2, value: "c" }];
    let raw_entry = RawEntry { index: table, entries };

    let _ref_mut = raw_entry.into_ref_mut();
    // Simulate parallel mutability attempt by trying to modify the table or entries
    let _table_clone = &mut raw_entry.index; // This should cause a panic
}

