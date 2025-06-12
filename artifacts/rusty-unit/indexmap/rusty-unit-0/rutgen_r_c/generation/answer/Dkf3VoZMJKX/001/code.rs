// Answer 0

#[test]
fn test_reserve_entries_success() {
    struct MyEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> MyEntries<K, V> {
        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
            let current_len = self.entries.len();
            if current_len + additional <= 10 {
                self.entries.reserve(additional);
                Ok(())
            } else {
                Err(TryReserveError)
            }
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }
    }

    let mut entries = MyEntries { entries: Vec::new() };
    // Pre-fill the entries with 5 elements
    for i in 0..5 {
        entries.entries.push(Bucket { hash: HashValue(0), key: i, value: i });
    }

    let additional = 4; // additional entries to reserve
    let try_capacity = 10; // maximum capacity in total

    reserve_entries(&mut entries.entries, additional, try_capacity);
    
    // Assert that we successfully reserved space for the additional entries
    assert_eq!(entries.entries.capacity(), 9); // 5 existing + 4 reserved
}

#[test]
#[should_panic]
fn test_reserve_entries_panics_on_exceeding_capacity() {
    struct MyEntries<K, V> {
        entries: Vec<Bucket<K, V>>,
    }

    impl<K, V> MyEntries<K, V> {
        fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
            let current_len = self.entries.len();
            if current_len + additional <= 5 {
                self.entries.reserve(additional);
                Ok(())
            } else {
                Err(TryReserveError)
            }
        }

        fn reserve_exact(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }
    }

    let mut entries = MyEntries { entries: Vec::new() };
    // Pre-fill the entries with 5 elements
    for i in 0..5 {
        entries.entries.push(Bucket { hash: HashValue(0), key: i, value: i });
    }

    let additional = 1; // we'd like to reserve one more
    let try_capacity = 6; // desired total capacity exceeds current

    reserve_entries(&mut entries.entries, additional, try_capacity); // This should panic
}

