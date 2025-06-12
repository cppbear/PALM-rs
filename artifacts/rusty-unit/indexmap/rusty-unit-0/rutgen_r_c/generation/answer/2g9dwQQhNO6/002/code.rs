// Answer 0

#[test]
fn test_swap_remove_index_none() {
    struct TestEntries {
        entries: Vec<Bucket<usize, String>>,
    }
    
    impl TestEntries {
        fn new() -> Self {
            Self { entries: Vec::new() }
        }

        fn get(&self, index: usize) -> Option<&Bucket<usize, String>> {
            self.entries.get(index)
        }

        fn swap_remove(&mut self, index: usize) -> Bucket<usize, String> {
            self.entries.swap_remove(index)
        }
    }

    let mut entries = TestEntries::new();
    let mut indices = hash_table::HashTable::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Test with an out-of-bounds index
    let index_to_remove = 0; // No entries added
    let result = ref_mut.swap_remove_index(index_to_remove);
    
    // Assert that the result is None
    assert!(result.is_none());
}

