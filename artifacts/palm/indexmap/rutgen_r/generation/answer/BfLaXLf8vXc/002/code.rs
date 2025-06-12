// Answer 0

#[test]
fn test_index_occupied() {
    struct OccupiedEntry {
        idx: usize,
    }

    impl OccupiedEntry {
        fn index(&self) -> usize {
            self.idx
        }
    }

    enum Entry {
        Occupied(OccupiedEntry),
        Vacant, // Not used in this test
    }

    impl Entry {
        pub fn index(&self) -> usize {
            match self {
                Self::Occupied(entry) => entry.index(),
                Self::Vacant => 0, // Not actually reached in this test
            }
        }
    }

    let occupied_entry = OccupiedEntry { idx: 5 };
    let entry = Entry::Occupied(occupied_entry);
    
    assert_eq!(entry.index(), 5);
}

#[test]
fn test_index_occupied_zero() {
    struct OccupiedEntry {
        idx: usize,
    }

    impl OccupiedEntry {
        fn index(&self) -> usize {
            self.idx
        }
    }

    enum Entry {
        Occupied(OccupiedEntry),
        Vacant, // Not used in this test
    }

    impl Entry {
        pub fn index(&self) -> usize {
            match self {
                Self::Occupied(entry) => entry.index(),
                Self::Vacant => 0, // Not actually reached in this test
            }
        }
    }

    let occupied_entry = OccupiedEntry { idx: 0 };
    let entry = Entry::Occupied(occupied_entry);
    
    assert_eq!(entry.index(), 0);
}

