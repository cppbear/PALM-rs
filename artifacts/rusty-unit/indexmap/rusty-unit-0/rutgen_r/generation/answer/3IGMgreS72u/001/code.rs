// Answer 0

#[test]
fn test_and_modify_when_entry_is_not_occupied() {
    struct OccupiedEntry<K, V> {
        key: K,
        value: V,
    }

    enum Entry<K, V> {
        Occupied(OccupiedEntry<K, V>),
        Vacant,
    }

    struct MyMap<K, V> {
        entry: Entry<K, V>,
    }

    impl<K, V> MyMap<K, V> {
        pub fn and_modify<F>(mut self, _f: F) -> Self
        where
            F: FnOnce(&mut K, &mut V),
        {
            if let Entry::Occupied(entry) = &mut self.entry {
                let (k, v) = (&mut entry.key, &mut entry.value);
                _f(k, v);
            }
            self
        }
    }

    let map = MyMap {
        entry: Entry::Vacant,
    };

    let result = map.and_modify(|_k, _v| {
        panic!("This should not be called");
    });

    assert!(matches!(result.entry, Entry::Vacant));
}

