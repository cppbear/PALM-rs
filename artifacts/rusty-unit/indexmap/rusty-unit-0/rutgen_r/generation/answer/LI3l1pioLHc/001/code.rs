// Answer 0

#[derive(Debug)]
struct VacantEntry<K> {
    key: K,
}

#[derive(Debug)]
enum Entry<K> {
    Occupied(VacantEntry<K>),
    Vacant(VacantEntry<K>),
}

impl<K> VacantEntry<K> {
    fn key(&self) -> &K {
        &self.key
    }
}

#[test]
fn test_key_vacant_entry() {
    let entry = Entry::Vacant(VacantEntry { key: "new_key" });

    match entry {
        Entry::Vacant(ref e) => {
            assert_eq!(e.key(), &"new_key");
        },
        _ => panic!("Expected a Vacant entry"),
    }
}

#[test]
fn test_key_occupied_entry() {
    let entry = Entry::Occupied(VacantEntry { key: "occupied_key" });

    match entry {
        Entry::Occupied(ref e) => {
            assert_eq!(e.key(), &"occupied_key");
        },
        _ => panic!("Expected an Occupied entry"),
    }
}

