// Answer 0

#[derive(Default)]
struct IndexMap {
    indices: Vec<usize>,
    entries: Vec<String>,
}

impl IndexMap {
    pub(crate) fn clear(&mut self) {
        self.indices.clear();
        self.entries.clear();
    }
}

#[test]
fn test_clear_empty_map() {
    let mut map = IndexMap::default();
    map.clear();
    assert!(map.indices.is_empty());
    assert!(map.entries.is_empty());
}

#[test]
fn test_clear_non_empty_map() {
    let mut map = IndexMap {
        indices: vec![0, 1, 2],
        entries: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    map.clear();
    assert!(map.indices.is_empty());
    assert!(map.entries.is_empty());
}

#[test]
fn test_clear_after_multiple_clears() {
    let mut map = IndexMap {
        indices: vec![1, 2, 3],
        entries: vec!["x".to_string(), "y".to_string(), "z".to_string()],
    };
    map.clear();
    assert!(map.indices.is_empty());
    assert!(map.entries.is_empty());
    map.clear(); // Clear again to test multiple clears
    assert!(map.indices.is_empty());
    assert!(map.entries.is_empty());
}

