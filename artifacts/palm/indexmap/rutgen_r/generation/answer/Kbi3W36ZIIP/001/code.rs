// Answer 0

#[test]
fn test_rebuild_hash_table_empty() {
    struct IndexMap {
        indices: Vec<usize>,
        entries: Vec<i32>,
    }

    impl IndexMap {
        fn new() -> Self {
            IndexMap {
                indices: Vec::new(),
                entries: Vec::new(),
            }
        }

        fn rebuild_hash_table(&mut self) {
            self.indices.clear();
            insert_bulk_no_grow(&mut self.indices, &self.entries);
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &Vec<i32>) {
        for (i, _entry) in entries.iter().enumerate() {
            indices.push(i);
        }
    }

    let mut index_map = IndexMap::new();
    index_map.rebuild_hash_table();
    assert!(index_map.indices.is_empty());
}

#[test]
fn test_rebuild_hash_table_single_entry() {
    struct IndexMap {
        indices: Vec<usize>,
        entries: Vec<i32>,
    }

    impl IndexMap {
        fn new(entries: Vec<i32>) -> Self {
            IndexMap {
                indices: Vec::new(),
                entries,
            }
        }

        fn rebuild_hash_table(&mut self) {
            self.indices.clear();
            insert_bulk_no_grow(&mut self.indices, &self.entries);
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &Vec<i32>) {
        for (i, _entry) in entries.iter().enumerate() {
            indices.push(i);
        }
    }

    let mut index_map = IndexMap::new(vec![42]);
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices, vec![0]);
}

#[test]
fn test_rebuild_hash_table_multiple_entries() {
    struct IndexMap {
        indices: Vec<usize>,
        entries: Vec<i32>,
    }

    impl IndexMap {
        fn new(entries: Vec<i32>) -> Self {
            IndexMap {
                indices: Vec::new(),
                entries,
            }
        }

        fn rebuild_hash_table(&mut self) {
            self.indices.clear();
            insert_bulk_no_grow(&mut self.indices, &self.entries);
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &Vec<i32>) {
        for (i, _entry) in entries.iter().enumerate() {
            indices.push(i);
        }
    }

    let mut index_map = IndexMap::new(vec![1, 2, 3]);
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices, vec![0, 1, 2]);
}

#[test]
fn test_rebuild_hash_table_no_growth() {
    struct IndexMap {
        indices: Vec<usize>,
        entries: Vec<i32>,
    }

    impl IndexMap {
        fn new(entries: Vec<i32>) -> Self {
            IndexMap {
                indices: Vec::with_capacity(entries.len()),
                entries,
            }
        }

        fn rebuild_hash_table(&mut self) {
            self.indices.clear();
            insert_bulk_no_grow(&mut self.indices, &self.entries);
        }
    }

    fn insert_bulk_no_grow(indices: &mut Vec<usize>, entries: &Vec<i32>) {
        for (i, _entry) in entries.iter().enumerate() {
            indices.push(i);
        }
    }

    let mut index_map = IndexMap::new(vec![10, 20, 30, 40]);
    index_map.rebuild_hash_table();
    assert_eq!(index_map.indices, vec![0, 1, 2, 3]);
}

