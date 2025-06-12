// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    struct OccupiedEntry<'a, K, V> {
        key: K,
        value: V,
        _marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a, K, V> OccupiedEntry<'a, K, V> {
        fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {
            (&mut self.key, &mut self.value)
        }
    }

    enum SelfEnum<K, V> {
        Occupied(OccupiedEntry<K, V>),
        Vacant,
    }

    impl<K, V> SelfEnum<K, V> {
        pub fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut K, &mut V),
        {
            if let Self::Occupied(entry) = &mut self {
                let (k, v) = entry.get_key_value_mut();
                f(k, v);
            }
            self
        }
    }

    // Initialize an occupied entry
    let mut entry = OccupiedEntry {
        key: 10,
        value: 20,
        _marker: std::marker::PhantomData,
    };

    // Wrap it into the SelfEnum
    let mut self_enum = SelfEnum::Occupied(entry);

    // Call the and_modify function with a closure that modifies the value
    self_enum = self_enum.and_modify(|k, v| {
        *k += 1; // Modify the key
        *v *= 2; // Modify the value
    });

    // Extract the entry to check modifications
    if let SelfEnum::Occupied(entry) = self_enum {
        assert_eq!(entry.key, 11); // Check that key was modified
        assert_eq!(entry.value, 40); // Check that value was modified
    } else {
        panic!("Expected an occupied entry after modification");
    }
}

