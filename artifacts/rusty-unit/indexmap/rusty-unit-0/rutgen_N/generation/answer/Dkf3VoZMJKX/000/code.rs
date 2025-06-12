// Answer 0

#[test]
fn test_reserve_entries_with_sufficient_capacity() {
    struct MockEntries {
        len: usize,
        capacity: usize,
    }

    impl MockEntries {
        fn new(len: usize, capacity: usize) -> Self {
            MockEntries { len, capacity }
        }
        
        fn len(&self) -> usize {
            self.len
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ()> {
            if self.len + additional <= self.capacity {
                self.len += additional;
                Ok(())
            } else {
                Err(())
            }
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.len += additional;
        }
    }

    let mut entries = MockEntries::new(5, 10);
    reserve_entries(&mut entries, 3, 10);
    assert_eq!(entries.len(), 8);
}

#[test]
fn test_reserve_entries_exceeding_capacity() {
    struct MockEntries {
        len: usize,
        capacity: usize,
    }

    impl MockEntries {
        fn new(len: usize, capacity: usize) -> Self {
            MockEntries { len, capacity }
        }
        
        fn len(&self) -> usize {
            self.len
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ()> {
            if self.len + additional <= self.capacity {
                self.len += additional;
                Ok(())
            } else {
                Err(())
            }
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.len += additional;
        }
    }

    let mut entries = MockEntries::new(5, 7);
    let additional = 5;
    let try_capacity = 10;

    reserve_entries(&mut entries, additional, try_capacity);
    assert_eq!(entries.len(), 10);
}

#[test]
fn test_reserve_entries_zero_additional() {
    struct MockEntries {
        len: usize,
        capacity: usize,
    }

    impl MockEntries {
        fn new(len: usize, capacity: usize) -> Self {
            MockEntries { len, capacity }
        }
        
        fn len(&self) -> usize {
            self.len
        }

        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), ()> {
            if self.len + additional <= self.capacity {
                self.len += additional;
                Ok(())
            } else {
                Err(())
            }
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.len += additional;
        }
    }

    let mut entries = MockEntries::new(3, 5);
    reserve_entries(&mut entries, 0, 5);
    assert_eq!(entries.len(), 3);
}

