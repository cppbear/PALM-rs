// Answer 0

#[test]
fn test_or_insert_vacant_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};
    
    #[derive(Hash)]
    struct Key {
        id: usize,
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            TestEntries { data: Vec::new() }
        }
        
        fn insert(&mut self, key: K, value: V) -> (&mut K, &mut V) {
            self.data.push((key, value));
            let last_index = self.data.len() - 1;
            let (k, v) = &mut self.data[last_index];
            (k, v)
        }
        
        fn get(&self, index: usize) -> Option<&(K, V)> {
            self.data.get(index)
        }
    }

    let entries = &mut TestEntries::new();
    let hasher = TestHasher;

    let vacant_entry = RawEntryMut::Vacant(RawVacantEntryMut {
        map: RefMut::new(entries),
        hash_builder: &hasher,
    });

    let key = Key { id: 1 };
    let value = "value1";

    let (key_ref, value_ref) = vacant_entry.or_insert(key, value);

    assert_eq!(key_ref.id, 1);
    assert_eq!(*value_ref, "value1");
}

#[test]
fn test_or_insert_occupied_entry() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    #[derive(Hash, PartialEq, Eq)]
    struct Key {
        id: usize,
    }

    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            TestEntries { data: Vec::new() }
        }

        fn insert(&mut self, key: K, value: V) -> (&mut K, &mut V) {
            self.data.push((key, value));
            let last_index = self.data.len() - 1;
            let (k, v) = &mut self.data[last_index];
            (k, v)
        }

        fn occupied_entry(&mut self) -> RawOccupiedEntryMut<Key, &str, TestHasher> {
            self.insert(Key { id: 2 }, "value2");
            RawOccupiedEntryMut {
                entries: self,
                index: 0,
                hash_builder: PhantomData,
            }
        }
    }

    let mut entries = TestEntries::new();
    let hasher = TestHasher;

    entries.insert(Key { id: 2 }, "value2");
    
    let occupied_entry = RawEntryMut::Occupied(entries.occupied_entry());

    let (key_ref, value_ref) = occupied_entry.or_insert(Key { id: 2 }, "default_value");

    assert_eq!(key_ref.id, 2);
    assert_eq!(*value_ref, "value2");
}

