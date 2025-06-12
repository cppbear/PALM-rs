// Answer 0

#[test]
fn test_or_default_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    struct HashMapEntry<'a, K, V> {
        map: &'a mut HashMap<K, Option<V>>,
        key: K,
    }

    impl<'a, K: std::hash::Hash + Eq, V: Default> HashMapEntry<'a, K, V> {
        fn or_default(self) -> &'a mut Option<V> {
            let entry = self.map.entry(self.key);
            match entry {
                Entry::Occupied(mut occupied) => occupied.into_mut(),
                Entry::Vacant(vacant) => vacant.insert(Some(V::default())),
            }
        }
    }

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    let entry = HashMapEntry { map: &mut map, key: "poneyland" };
    assert_eq!(entry.or_default(), &mut Some(0));
    assert_eq!(map["poneyland"], Some(0));
}

#[test]
fn test_or_default_with_existing_key() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    struct HashMapEntry<'a, K, V> {
        map: &'a mut HashMap<K, Option<V>>,
        key: K,
    }

    impl<'a, K: std::hash::Hash + Eq, V: Default> HashMapEntry<'a, K, V> {
        fn or_default(self) -> &'a mut Option<V> {
            let entry = self.map.entry(self.key);
            match entry {
                Entry::Occupied(mut occupied) => occupied.into_mut(),
                Entry::Vacant(vacant) => vacant.insert(Some(V::default())),
            }
        }
    }

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    map.insert("horseland", Some(3));
    let entry = HashMapEntry { map: &mut map, key: "horseland" };
    assert_eq!(entry.or_default(), &mut Some(3));
}

#[test]
#[should_panic]
fn test_or_default_panic_on_invalid_type() {
    use hashbrown::HashMap;
    use std::collections::hash_map::Entry;

    struct HashMapEntry<'a, K, V> {
        map: &'a mut HashMap<K, V>,
        key: K,
    }

    impl<'a, K: std::hash::Hash + Eq, V> HashMapEntry<'a, K, V> {
        fn or_default(self) -> &'a mut V {
            let entry = self.map.entry(self.key);
            match entry {
                Entry::Occupied(mut occupied) => occupied.into_mut(),
                Entry::Vacant(vacant) => vacant.insert(0), // Assuming V != Option
            }
        }
    }

    let mut map: HashMap<&str, u32> = HashMap::new();
    let entry = HashMapEntry { map: &mut map, key: "invalid" };
    let _ = entry.or_default(); // This will panic due to incorrect type
}

