// Answer 0

#[derive(Debug)]
struct OccupiedEntry<K> {
    key: K,
}

impl<K> OccupiedEntry<K> {
    fn key(&self) -> &K {
        &self.key
    }
}

#[derive(Debug)]
struct VacantEntry<K> {
    key: K,
}

impl<K> VacantEntry<K> {
    fn key(&self) -> &K {
        &self.key
    }
}

enum Entry<K> {
    Occupied(OccupiedEntry<K>),
    Vacant(VacantEntry<K>),
}

#[test]
fn test_entry_key_occupied() {
    let occupied_entry = OccupiedEntry { key: 42 };
    let entry = Entry::Occupied(occupied_entry);

    match entry {
        Entry::Occupied(ref entry) => {
            assert_eq!(entry.key(), &42);
        },
        Entry::Vacant(_) => panic!("Expected an occupied entry"),
    }
}

#[test]
fn test_entry_key_vacant() {
    let vacant_entry = VacantEntry { key: 100 };
    let entry = Entry::Vacant(vacant_entry);

    match entry {
        Entry::Occupied(_) => panic!("Expected a vacant entry"),
        Entry::Vacant(ref entry) => {
            assert_eq!(entry.key(), &100);
        },
    }
}

