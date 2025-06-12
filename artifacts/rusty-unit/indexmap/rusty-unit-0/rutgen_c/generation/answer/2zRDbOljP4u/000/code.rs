// Answer 0

#[test]
fn test_move_index_shift_down() {
    struct MockEntry;
    
    impl Entries for MockEntry {
        type Entry = (usize, usize);
        fn into_entries(self) -> Vec<Self::Entry> {
            vec![(0, 0), (1, 1), (2, 2)]
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &[(0, 0), (1, 1), (2, 2)]
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut [(0, 0), (1, 1), (2, 2)]
        }
        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut entries = MockEntry;
    let index = 1;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    occupied_entry.move_index(2);
    
    assert_eq!(occupied_entry.index(), 2);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds() {
    struct MockEntry;

    impl Entries for MockEntry {
        type Entry = (usize, usize);
        fn into_entries(self) -> Vec<Self::Entry> {
            vec![(0, 0), (1, 1), (2, 2)]
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &[(0, 0), (1, 1), (2, 2)]
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut [(0, 0), (1, 1), (2, 2)]
        }
        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut entries = MockEntry;
    let index = 1;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    occupied_entry.move_index(5); // This should panic
}

#[test]
fn test_move_index_shift_up() {
    struct MockEntry;

    impl Entries for MockEntry {
        type Entry = (usize, usize);
        fn into_entries(self) -> Vec<Self::Entry> {
            vec![(0, 0), (1, 1), (2, 2)]
        }
        fn as_entries(&self) -> &[Self::Entry] {
            &[(0, 0), (1, 1), (2, 2)]
        }
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut [(0, 0), (1, 1), (2, 2)]
        }
        fn with_entries<F>(&mut self, _f: F) where F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut entries = MockEntry;
    let index = 2;
    let occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::from_index(index),
        hash_builder: PhantomData,
    };

    occupied_entry.move_index(1);
    
    assert_eq!(occupied_entry.index(), 1);
}

