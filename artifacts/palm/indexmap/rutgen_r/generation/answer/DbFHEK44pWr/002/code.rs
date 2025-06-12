// Answer 0

#[test]
fn test_or_insert_with_key_occupied() {
    struct MyEntry<K, V> {
        key: K,
        value: V,
    }

    enum Entry<K, V> {
        Occupied(MyEntry<K, V>),
        Vacant(MyEntry<K, V>), // For the sake of the test, not used here
    }

    impl<K, V> Entry<K, V> {
        pub fn or_insert_with_key<F>(&mut self, call: F) -> &mut V
        where
            F: FnOnce(&K) -> V,
        {
            match self {
                Entry::Occupied(entry) => &mut entry.value,
                Entry::Vacant(_) => panic!("Entry is vacant"),
            }
        }
    }

    let mut occupied_entry = Entry::Occupied(MyEntry { key: "key1", value: 42 });
    let value = occupied_entry.or_insert_with_key(|key| *key.len() as i32);
    assert_eq!(*value, 42);
}

