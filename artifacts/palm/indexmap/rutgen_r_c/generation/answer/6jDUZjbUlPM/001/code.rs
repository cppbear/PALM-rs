// Answer 0

#[test]
fn test_swap_remove_empty() {
    struct SimpleEntries {
        data: Vec<(i32, i32)>,
    }

    impl Entries for SimpleEntries {
        type Entry = (i32, i32);
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = SimpleEntries { data: Vec::new() };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::empty());

    let result = std::panic::catch_unwind(|| {
        occupied_entry.swap_remove()
    });

    assert!(result.is_err());
}

#[test]
fn test_swap_remove_one_element() {
    struct SimpleEntries {
        data: Vec<(i32, i32)>,
    }

    impl Entries for SimpleEntries {
        type Entry = (i32, i32);
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = SimpleEntries { data: vec![(1, 100)] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::from_index(0));

    let result = occupied_entry.swap_remove();
    assert_eq!(result, 100);
    assert!(entries.data.is_empty());
}

#[test]
fn test_swap_remove_multiple_elements() {
    struct SimpleEntries {
        data: Vec<(i32, i32)>,
    }

    impl Entries for SimpleEntries {
        type Entry = (i32, i32);
        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = SimpleEntries { data: vec![(1, 100), (2, 200), (3, 300)] };
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::from_index(1));

    let result = occupied_entry.swap_remove();
    assert_eq!(result, 200);
    assert_eq!(entries.data.len(), 2);
    assert_eq!(entries.data[0], (1, 100));
    assert_eq!(entries.data[1], (3, 300));
}

