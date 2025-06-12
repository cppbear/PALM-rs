// Answer 0

#[derive(Debug)]
struct OccupiedEntry {
    index_value: usize,
}

impl OccupiedEntry {
    fn index(&self) -> usize {
        self.index_value
    }
}

#[derive(Debug)]
struct VacantEntry {
    index_value: usize,
}

impl VacantEntry {
    fn index(&self) -> usize {
        self.index_value
    }
}

enum Entry {
    Occupied(OccupiedEntry),
    Vacant(VacantEntry),
}

impl Entry {
    pub fn index(&self) -> usize {
        match self {
            Self::Occupied(entry) => entry.index(),
            Self::Vacant(entry) => entry.index(),
        }
    }
}

#[test]
fn test_occupied_entry_index() {
    let occupied_entry = OccupiedEntry { index_value: 5 };
    let entry = Entry::Occupied(occupied_entry);
    assert_eq!(entry.index(), 5);
}

#[test]
fn test_vacant_entry_index() {
    let vacant_entry = VacantEntry { index_value: 10 };
    let entry = Entry::Vacant(vacant_entry);
    assert_eq!(entry.index(), 10);
}

