// Answer 0

#[derive(Default)]
struct TestMap {
    entries: Vec<i32>,
}

impl TestMap {
    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Self::Entry]),
    {
        f(&mut self.entries);
    }
}

#[test]
fn test_with_entries_empty() {
    let mut map = TestMap::default();
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 0);
    });
}

#[test]
fn test_with_entries_single_entry() {
    let mut map = TestMap { entries: vec![1] };
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], 1);
    });
}

#[test]
fn test_with_entries_multiple_entries() {
    let mut map = TestMap { entries: vec![1, 2, 3] };
    map.with_entries(|entries| {
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], 1);
        assert_eq!(entries[1], 2);
        assert_eq!(entries[2], 3);
    });
}

