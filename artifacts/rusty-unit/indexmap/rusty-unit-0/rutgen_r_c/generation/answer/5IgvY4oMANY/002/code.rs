// Answer 0

#[test]
fn test_swap_remove_finish_valid_case() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn swap_remove(&mut self, index: usize) -> Bucket<usize, usize> {
            self.entries.swap_remove(index)
        }

        fn get(&self, index: usize) -> Option<&Bucket<usize, usize>> {
            self.entries.get(index)
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let mut indices = hash_table::HashTable::new();
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: 10, value: 100 },
            Bucket { hash: HashValue(2), key: 20, value: 200 },
            Bucket { hash: HashValue(3), key: 30, value: 300 },
        ],
    };

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let (key, value) = ref_mut.swap_remove_finish(1);
    assert_eq!(key, 20);
    assert_eq!(value, 200);
    assert_eq!(entries.len(), 2);
}

#[test]
#[should_panic]
fn test_swap_remove_finish_out_of_bounds() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn swap_remove(&mut self, index: usize) -> Bucket<usize, usize> {
            self.entries.swap_remove(index)
        }
        
        fn len(&self) -> usize {
            self.entries.len()
        }
        
        fn get(&self, index: usize) -> Option<&Bucket<usize, usize>> {
            self.entries.get(index)
        }
    }

    let mut indices = hash_table::HashTable::new();
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: 10, value: 100 },
            Bucket { hash: HashValue(2), key: 20, value: 200 },
            Bucket { hash: HashValue(3), key: 30, value: 300 },
        ],
    };

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // This should panic because index 5 is out of bounds
    ref_mut.swap_remove_finish(5);
}

#[test]
fn test_swap_remove_finish_with_last_element() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl TestEntries {
        fn swap_remove(&mut self, index: usize) -> Bucket<usize, usize> {
            self.entries.swap_remove(index)
        }

        fn get(&self, index: usize) -> Option<&Bucket<usize, usize>> {
            self.entries.get(index)
        }

        fn len(&self) -> usize {
            self.entries.len()
        }
    }

    let mut indices = hash_table::HashTable::new();
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: 10, value: 100 },
        ],
    };

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let (key, value) = ref_mut.swap_remove_finish(0);
    assert_eq!(key, 10);
    assert_eq!(value, 100);
    assert_eq!(entries.len(), 0);
}

