// Answer 0

#[derive(Debug)]
struct OccupiedEntry {
    key: i32,
}

impl OccupiedEntry {
    fn key_mut(&mut self) -> &mut i32 {
        &mut self.key
    }
}

#[derive(Debug)]
struct VacantEntry {
    key: i32,
}

impl VacantEntry {
    fn key_mut(&mut self) -> &mut i32 {
        &mut self.key
    }
}

enum Entry {
    Occupied(OccupiedEntry),
    Vacant(VacantEntry),
}

impl Entry {
    fn key_mut(&mut self) -> &mut i32 {
        match self {
            Entry::Occupied(e) => e.key_mut(),
            Entry::Vacant(e) => e.key_mut(),
        }
    }
}

#[test]
fn test_key_mut_occupied() {
    let mut entry = Entry::Occupied(OccupiedEntry { key: 10 });
    *entry.key_mut() = 20;
    if let Entry::Occupied(ref occupied) = entry {
        assert_eq!(occupied.key, 20);
    }
}

#[test]
fn test_key_mut_vacant() {
    let mut entry = Entry::Vacant(VacantEntry { key: 30 });
    *entry.key_mut() = 40;
    if let Entry::Vacant(ref vacant) = entry {
        assert_eq!(vacant.key, 40);
    }
}

