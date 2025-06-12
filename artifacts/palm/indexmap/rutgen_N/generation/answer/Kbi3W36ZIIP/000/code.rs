// Answer 0

#[test]
fn test_rebuild_hash_table() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                indices: Vec::new(),
                entries: Vec::new(),
            }
        }

        fn rebuild_hash_table(&mut self) {
            self.indices.clear();
            insert_bulk_no_grow(&mut self.indices, &self.entries);
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &[String]) {
        indices.extend(0..entries.len());
    }

    let mut map = TestMap::new();
    map.entries.push("first".to_string());
    map.entries.push("second".to_string());

    map.rebuild_hash_table();

    assert_eq!(map.indices.len(), 2);
    assert_eq!(map.indices, vec![0, 1]);
}

#[test]
fn test_rebuild_hash_table_empty() {
    struct TestMap {
        indices: Vec<usize>,
        entries: Vec<String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                indices: Vec::new(),
                entries: Vec::new(),
            }
        }

        fn rebuild_hash_table(&mut self) {
            self.indices.clear();
            insert_bulk_no_grow(&mut self.indices, &self.entries);
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &[String]) {
        indices.extend(0..entries.len());
    }

    let mut map = TestMap::new();
    map.rebuild_hash_table();

    assert_eq!(map.indices.len(), 0);
    assert_eq!(map.indices, Vec::<usize>::new());
}

