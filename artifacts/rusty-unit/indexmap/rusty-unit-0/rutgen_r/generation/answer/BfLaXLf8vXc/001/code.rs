// Answer 0

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

struct OccupiedEntry {
    index_value: usize,
}

impl OccupiedEntry {
    fn index(&self) -> usize {
        self.index_value
    }
}

#[test]
fn test_vacant_entry_index() {
    let vacant_entry = VacantEntry { index_value: 42 };
    let entry = Entry::Vacant(vacant_entry);
    assert_eq!(entry.index(), 42);
}

#[test]
fn test_vacant_entry_index_zero() {
    let vacant_entry = VacantEntry { index_value: 0 };
    let entry = Entry::Vacant(vacant_entry);
    assert_eq!(entry.index(), 0);
}

#[test]
fn test_vacant_entry_index_boundary() {
    let vacant_entry = VacantEntry { index_value: usize::MAX };
    let entry = Entry::Vacant(vacant_entry);
    assert_eq!(entry.index(), usize::MAX);
}

