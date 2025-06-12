// Answer 0

#[test]
fn test_erase_indices_with_no_erased_entries() {
    struct TestEntries;
    
    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        fn into_entries(self) -> Vec<Self::Entry> { Vec::new() }
        fn as_entries(&self) -> &[Self::Entry] { &[] }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] { &mut [] }
        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut index_map = IndexMapCore::<usize, usize> {
        indices: Indices::new(),
        entries: TestEntries,
    };

    index_map.erase_indices(0, 0);
    
    assert_eq!(index_map.indices.len(), 0);
}

#[test]
fn test_erase_indices_with_erased_entries() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> { self.entries }
        
        fn as_entries(&self) -> &[Self::Entry] { &self.entries }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] { &mut self.entries }
        
        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut index_map = IndexMapCore {
        indices: Indices::with_capacity(5),
        entries: TestEntries { entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: 10 },
            Bucket { hash: HashValue(2), key: 2, value: 20 },
            Bucket { hash: HashValue(3), key: 3, value: 30 },
            Bucket { hash: HashValue(4), key: 4, value: 40 },
        ]},
    };
    
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());
    index_map.indices.insert_unique(4, 3, |_| unreachable!());

    index_map.erase_indices(1, 3);
    
    assert_eq!(index_map.indices.len(), 2);
    assert_eq!(index_map.indices.as_entries()[0].key, 1);
    assert_eq!(index_map.indices.as_entries()[1].key, 4);
}

#[test]
fn test_erase_indices_with_full_table() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> { self.entries }
        
        fn as_entries(&self) -> &[Self::Entry] { &self.entries }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] { &mut self.entries }
        
        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut index_map = IndexMapCore {
        indices: Indices::with_capacity(5),
        entries: TestEntries { entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: 10 },
            Bucket { hash: HashValue(2), key: 2, value: 20 },
            Bucket { hash: HashValue(3), key: 3, value: 30 },
            Bucket { hash: HashValue(4), key: 4, value: 40 },
            Bucket { hash: HashValue(5), key: 5, value: 50 },
        ]},
    };
    
    index_map.indices.insert_unique(1, 0, |_| unreachable!());
    index_map.indices.insert_unique(2, 1, |_| unreachable!());
    index_map.indices.insert_unique(3, 2, |_| unreachable!());
    index_map.indices.insert_unique(4, 3, |_| unreachable!());
    index_map.indices.insert_unique(5, 4, |_| unreachable!());

    index_map.erase_indices(0, 5);
    
    assert_eq!(index_map.indices.len(), 0);
}

