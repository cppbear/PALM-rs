// Answer 0

#[test]
fn test_swap_remove() {
    struct TestEntries {
        entries: Vec<usize>,
    }

    impl TestEntries {
        fn new() -> Self {
            Self { entries: vec![] }
        }

        fn insert(&mut self, key: usize) {
            self.entries.push(key);
        }

        fn occupied_entry(&mut self, index: usize) -> OccupiedEntry<usize, usize> {
            let index_entry = hash_table::OccupiedEntry::new(&mut self.entries, index);
            OccupiedEntry::new(self, index_entry)
        }
    }

    let mut map = TestEntries::new();
    map.insert(10);
    map.insert(20);
    map.insert(30);

    let occupied = map.occupied_entry(1); // Occupy index with key 20
    let result = occupied.swap_remove();

    assert_eq!(result, 20);
    assert_eq!(map.entries.len(), 2);
    assert_eq!(map.entries[1], 30); // Ensure 20 was swapped out
}

