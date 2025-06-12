// Answer 0

#[derive(Debug, Hash, Eq, PartialEq)]
struct Entry {
    key: String,
    value: i32,
}

struct TestMap {
    entries: Vec<Entry>,
}

impl TestMap {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn insert(&mut self, key: String, value: i32) {
        self.entries.push(Entry { key, value });
    }

    fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(String, i32)>
    where
        Q: ?Sized + Hash + Equivalent<String>,
    {
        match self.shift_remove_full(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, String, i32)>
    where
        Q: ?Sized + Hash + Equivalent<String>,
    {
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.key == *key {
                let removed_entry = self.entries.remove(index);
                return Some((index, removed_entry.key, removed_entry.value));
            }
        }
        None
    }
}

#[test]
fn test_shift_remove_entry_existing_key() {
    let mut map = TestMap::new();
    map.insert("foo".to_string(), 42);
    map.insert("bar".to_string(), 37);

    let result = map.shift_remove_entry("foo");
    assert_eq!(result, Some(("foo".to_string(), 42)));
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key, "bar");
}

#[test]
fn test_shift_remove_entry_non_existing_key() {
    let mut map = TestMap::new();
    map.insert("foo".to_string(), 42);

    let result = map.shift_remove_entry("baz");
    assert_eq!(result, None);
    assert_eq!(map.entries.len(), 1);
}

#[test]
fn test_shift_remove_entry_boundary_condition() {
    let mut map = TestMap::new();
    map.insert("only".to_string(), 99);

    let result = map.shift_remove_entry("only");
    assert_eq!(result, Some(("only".to_string(), 99)));
    assert_eq!(map.entries.len(), 0);
}

