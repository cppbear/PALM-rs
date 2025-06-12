// Answer 0

#[test]
fn test_index_occupied_entry() {
    struct OccupiedEntry {
        index: usize,
    }

    impl OccupiedEntry {
        fn index(&self) -> usize {
            self.index
        }
    }

    enum Entry {
        Occupied(OccupiedEntry),
        Vacant,
    }

    let entry = Entry::Occupied(OccupiedEntry { index: 5 });
    match entry {
        Entry::Occupied(ref e) => assert_eq!(e.index(), 5),
        Entry::Vacant => panic!("Expected occupied entry"),
    }
}

#[test]
fn test_index_vacant_entry() {
    struct VacantEntry {
        index: usize,
    }

    impl VacantEntry {
        fn index(&self) -> usize {
            self.index
        }
    }

    enum Entry {
        Occupied(VacantEntry),
        Vacant(VacantEntry),
    }

    let entry = Entry::Vacant(VacantEntry { index: 10 });
    match entry {
        Entry::Vacant(ref e) => assert_eq!(e.index(), 10),
        Entry::Occupied => panic!("Expected vacant entry"),
    }
}

