// Answer 0

#[test]
fn test_try_reserve_entries_exact_capacity() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for TestEntries {
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

    let mut map = IndexMapCore::<usize, usize>::with_capacity(5);
    let current_len = map.entries.len();
    let additional = 5; // This will match the available capacity.

    // Reserve to meet the current capacity conditions
    let result = map.try_reserve_entries(additional);

    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "requested capacity exceeds maximum limit")]
fn test_try_reserve_entries_exceeding_capacity() {
    #[derive(Debug)]
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }

    impl Entries for TestEntries {
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

    let mut map = IndexMapCore::<usize, usize>::with_capacity(5);
    map.entries.extend_from_slice(&[Bucket { hash: HashValue::default(), key: 1, value: 10 }; 5]);
    let additional = 10; // This exceeds the max capacity defined.

    // Expect panic due to exceedance in capacity
    let _ = map.try_reserve_entries(additional);
}

