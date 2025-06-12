// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct DummyEntries {
        data: Vec<usize>,
    }

    impl Entries for DummyEntries {
        type Entry = usize;

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

    let mut entries = DummyEntries { data: vec![1, 2, 3, 4] };
    let index_entry = hash_table::OccupiedEntry::new(1); // Arbitrarily taking the second element
    let occupied_entry = OccupiedEntry::new(&mut entries, index_entry);

    occupied_entry.swap_indices(2); // Swap with the third element

    assert_eq!(entries.data, vec![1, 3, 2, 4]);
}

#[should_panic]
#[test]
fn test_swap_indices_out_of_bounds() {
    struct DummyEntries {
        data: Vec<usize>,
    }

    impl Entries for DummyEntries {
        type Entry = usize;

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

    let mut entries = DummyEntries { data: vec![1, 2, 3] };
    let index_entry = hash_table::OccupiedEntry::new(1); // Taking the second element
    let occupied_entry = OccupiedEntry::new(&mut entries, index_entry);
    
    occupied_entry.swap_indices(3); // This should panic as there is no fourth element
}

