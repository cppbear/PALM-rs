// Answer 0

#[test]
fn test_remove_entry_existing() {
    struct Dummy {
        value: i32,
    }

    struct DummyTable {
        entries: Vec<Dummy>,
    }

    impl DummyTable {
        fn new() -> Self {
            DummyTable { entries: Vec::new() }
        }

        fn insert(&mut self, hash: u64, value: i32) {
            self.entries.push(Dummy { value });
        }

        fn find<F>(&self, hash: u64, mut eq: F) -> Option<usize>
        where
            F: FnMut(&Dummy) -> bool,
        {
            self.entries.iter().position(|entry| eq(entry))
        }

        unsafe fn remove(&mut self, index: usize) -> (Dummy, usize) {
            let entry = self.entries.remove(index);
            (entry, index)
        }

        pub fn remove_entry(&mut self, hash: u64, eq: impl FnMut(&Dummy) -> bool) -> Option<Dummy> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { self.remove(bucket).0 }),
                None => None,
            }
        }
    }

    let mut table = DummyTable::new();
    table.insert(1, 42);
    table.insert(2, 100);
    
    let removed_entry = table.remove_entry(1, |e| e.value == 42);
    
    assert!(removed_entry.is_some());
    assert_eq!(removed_entry.unwrap().value, 42);
}

#[test]
fn test_remove_entry_non_existing() {
    struct Dummy {
        value: i32,
    }

    struct DummyTable {
        entries: Vec<Dummy>,
    }

    impl DummyTable {
        fn new() -> Self {
            DummyTable { entries: Vec::new() }
        }

        fn insert(&mut self, hash: u64, value: i32) {
            self.entries.push(Dummy { value });
        }

        fn find<F>(&self, hash: u64, mut eq: F) -> Option<usize>
        where
            F: FnMut(&Dummy) -> bool,
        {
            self.entries.iter().position(|entry| eq(entry))
        }

        unsafe fn remove(&mut self, index: usize) -> (Dummy, usize) {
            let entry = self.entries.remove(index);
            (entry, index)
        }

        pub fn remove_entry(&mut self, hash: u64, eq: impl FnMut(&Dummy) -> bool) -> Option<Dummy> {
            match self.find(hash, eq) {
                Some(bucket) => Some(unsafe { self.remove(bucket).0 }),
                None => None,
            }
        }
    }

    let mut table = DummyTable::new();
    table.insert(1, 42);
    table.insert(2, 100);
    
    let removed_entry = table.remove_entry(3, |e| e.value == 200);
    
    assert!(removed_entry.is_none());
}

