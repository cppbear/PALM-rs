// Answer 0

#[derive(Debug)]
struct OccupiedEntry {
    index: usize,
}

impl OccupiedEntry {
    fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug)]
struct VacantEntry {
    index: usize,
}

impl VacantEntry {
    fn index(&self) -> usize {
        self.index
    }
}

enum Entry {
    Occupied(OccupiedEntry),
    Vacant(VacantEntry),
}

impl Entry {
    pub fn index(&self) -> usize {
        match *self {
            Entry::Occupied(ref entry) => entry.index(),
            Entry::Vacant(ref entry) => entry.index(),
        }
    }
}

#[test]
fn test_index_occupied_entry() {
    let occupied_entry = OccupiedEntry { index: 5 };
    let entry = Entry::Occupied(occupied_entry);
    assert_eq!(entry.index(), 5);
}

#[test]
fn test_index_vacant_entry() {
    let vacant_entry = VacantEntry { index: 10 };
    let entry = Entry::Vacant(vacant_entry);
    assert_eq!(entry.index(), 10);
}

