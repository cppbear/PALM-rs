// Answer 0

#[test]
fn test_erase_index_existing_entry() {
    struct TestHashTable {
        storage: Vec<(u64, usize)>,
    }

    impl TestHashTable {
        fn new() -> Self {
            TestHashTable { storage: Vec::new() }
        }

        fn find_entry(&mut self, hash: u64, f: impl Fn(&usize) -> bool) -> Result<OccupiedEntry, ()> {
            for (h, &index) in &self.storage {
                if *h == hash && f(&index) {
                    return Ok(OccupiedEntry { index });
                }
            }
            Err(())
        }

        fn insert(&mut self, hash: u64, index: usize) {
            self.storage.push((hash, index));
        }
    }

    struct OccupiedEntry {
        index: usize,
    }

    impl OccupiedEntry {
        fn remove(self) {
            // Logic to simulate removing an entry
        }
    }

    let mut table = TestHashTable::new();
    let hash = HashValue(42);
    
    table.insert(hash.get(), 1);
    
    erase_index(&mut table, hash, 1);
}

#[test]
#[should_panic(expected = "index not found")]
fn test_erase_index_non_existing_entry() {
    struct TestHashTable {
        storage: Vec<(u64, usize)>,
    }

    impl TestHashTable {
        fn new() -> Self {
            TestHashTable { storage: Vec::new() }
        }

        fn find_entry(&mut self, hash: u64, f: impl Fn(&usize) -> bool) -> Result<OccupiedEntry, ()> {
            for (h, &index) in &self.storage {
                if *h == hash && f(&index) {
                    return Ok(OccupiedEntry { index });
                }
            }
            Err(())
        }

        fn insert(&mut self, hash: u64, index: usize) {
            self.storage.push((hash, index));
        }
    }

    struct OccupiedEntry {
        index: usize,
    }

    impl OccupiedEntry {
        fn remove(self) {
            // Logic to simulate removing an entry
        }
    }

    let mut table = TestHashTable::new();
    let hash = HashValue(42);
    
    table.insert(hash.get(), 1);
    
    // Invalid entry removal attempt
    erase_index(&mut table, hash, 2);
}

