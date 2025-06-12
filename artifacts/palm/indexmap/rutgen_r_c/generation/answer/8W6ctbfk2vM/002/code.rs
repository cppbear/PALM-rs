// Answer 0

#[test]
fn test_try_reserve_entries_success() {
    struct MockEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for MockEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut index_map = IndexMapCore {
        indices: Indices::new(),
        entries: MockEntries { entries: Vec::new() },
    };

    // Set up initial conditions
    let initial_capacity = 10;
    for _ in 0..initial_capacity {
        index_map.entries.as_entries_mut().push(Bucket { hash: HashValue::new(0), key: 0, value: 0 });
    }

    // Define additional size that we want to reserve
    let additional = 5;

    // By setting capacity to be equal to the current length, make try_reserve_exact fail
    index_map.indices = Indices::with_capacity(initial_capacity);

    let result = index_map.try_reserve_entries(additional);
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_entries_failure() {
    struct MockEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for MockEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut index_map = IndexMapCore {
        indices: Indices::new(),
        entries: MockEntries { entries: Vec::new() },
    };

    // Set up initial conditions
    let initial_capacity = 10;
    for _ in 0..initial_capacity {
        index_map.entries.as_entries_mut().push(Bucket { hash: HashValue::new(0), key: 0, value: 0 });
    }

    // Define additional size that we want to reserve
    let additional = 15; // Enough to exceed the total capacity

    // Set capacity to be equal to the current size to ensure failure when trying to reserve
    index_map.indices = Indices::with_capacity(initial_capacity);

    let result = index_map.try_reserve_entries(additional);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_try_reserve_entries_panic() {
    struct MockEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for MockEntries {
        type Entry = Bucket<usize, usize>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut index_map = IndexMapCore {
        indices: Indices::new(),
        entries: MockEntries{ entries: Vec::new() },
    };

    // Set up initial conditions
    let initial_capacity = 10;
    for _ in 0..initial_capacity {
        index_map.entries.as_entries_mut().push(Bucket { hash: HashValue::new(0), key: 0, value: 0 });
    }

    // Attempt to reserve more capacity than is available, expecting panic
    let additional = 100;
    index_map.try_reserve_entries(additional);
}

