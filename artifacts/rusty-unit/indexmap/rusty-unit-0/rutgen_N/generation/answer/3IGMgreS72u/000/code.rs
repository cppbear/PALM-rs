// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    struct OccupiedEntry<K, V> {
        key: K,
        value: V,
    }

    enum RawEntry<K, V> {
        Occupied(OccupiedEntry<K, V>),
        Vacant,
    }

    impl<K, V> RawEntry<K, V> {
        pub fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut K, &mut V),
        {
            if let Self::Occupied(entry) = &mut self {
                let (k, v) = (&mut entry.key, &mut entry.value);
                f(k, v);
            }
            self
        }
    }

    let entry = RawEntry::Occupied(OccupiedEntry { key: 1, value: 2 });
    let modified_entry = entry.and_modify(|k, v| {
        *k += 1;
        *v *= 2;
    });

    if let RawEntry::Occupied(ref e) = modified_entry {
        assert_eq!(e.key, 2);
        assert_eq!(e.value, 4);
    } else {
        panic!("Expected entry to be occupied");
    }
}

#[test]
fn test_and_modify_vacant_entry() {
    struct OccupiedEntry<K, V> {
        key: K,
        value: V,
    }

    enum RawEntry<K, V> {
        Occupied(OccupiedEntry<K, V>),
        Vacant,
    }

    impl<K, V> RawEntry<K, V> {
        pub fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut K, &mut V),
        {
            if let Self::Occupied(entry) = &mut self {
                let (k, v) = (&mut entry.key, &mut entry.value);
                f(k, v);
            }
            self
        }
    }

    let entry = RawEntry::Vacant;
    let modified_entry = entry.and_modify(|_k, _v| {
        panic!("This closure should not be called");
    });

    match modified_entry {
        RawEntry::Vacant => (),
        RawEntry::Occupied(_) => panic!("Expected entry to remain vacant"),
    }
}

