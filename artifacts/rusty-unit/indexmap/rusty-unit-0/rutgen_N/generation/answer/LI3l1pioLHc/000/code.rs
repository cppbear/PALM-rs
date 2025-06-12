// Answer 0

#[derive(Debug)]
struct OccupiedEntry<K> {
    key: K,
}

#[derive(Debug)]
struct VacantEntry<K> {
    key: K,
}

enum Entry<K> {
    Occupied(OccupiedEntry<K>),
    Vacant(VacantEntry<K>),
}

impl<K> OccupiedEntry<K> {
    fn key(&self) -> &K {
        &self.key
    }
}

impl<K> VacantEntry<K> {
    fn key(&self) -> &K {
        &self.key
    }
}

impl<K> Entry<K> {
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }
}

#[test]
fn test_key_occupied_entry() {
    let occupied_entry = Entry::Occupied(OccupiedEntry { key: "occupied_key" });
    assert_eq!(occupied_entry.key(), &"occupied_key");
}

#[test]
fn test_key_vacant_entry() {
    let vacant_entry = Entry::Vacant(VacantEntry { key: "vacant_key" });
    assert_eq!(vacant_entry.key(), &"vacant_key");
}

