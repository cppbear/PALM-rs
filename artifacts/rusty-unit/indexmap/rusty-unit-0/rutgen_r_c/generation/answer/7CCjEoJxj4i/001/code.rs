// Answer 0

#[test]
fn test_insert_hashed_nocheck() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct MockEntries<K, V> {
        data: Vec<(K, V)>,
        capacity: usize,
    }

    impl<K, V> MockEntries<K, V> {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn push(&mut self, key: K, value: V) {
            self.data.push((key, value));
        }

        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
        
        fn get_mut(&mut self, index: usize) -> &mut (K, V) {
            &mut self.data[index]
        }
    }

    let mut indices = vec![];
    let mut entries = MockEntries::new(2);
    
    {
        let hasher = MockHasher;
        let mut ref_mut = RefMut::new(&mut indices, &mut entries);

        let (key_ref, value_ref) = RawVacantEntryMut {
            map: ref_mut,
            hash_builder: &hasher,
        }
        .insert_hashed_nocheck(42, "test_key", "test_value");

        assert_eq!(unsafe { &*key_ref }, "test_key");
        assert_eq!(unsafe { &*value_ref }, "test_value");
        
        // Check that the entry was added
        assert_eq!(entries.len(), 1);
        assert_eq!(entries.get_mut(0), &mut ("test_key", "test_value"));
    }
}

#[test]
#[should_panic]
fn test_insert_hashed_nocheck_capacity() {
    struct MockHasher;

    impl BuildHasher for MockHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct MockEntries<K, V> {
        data: Vec<(K, V)>,
        capacity: usize,
    }

    impl<K, V> MockEntries<K, V> {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn push(&mut self, key: K, value: V) {
            if self.len() >= self.capacity {
                panic!("Capacity exceeded");
            }
            self.data.push((key, value));
        }

        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn capacity(&self) -> usize {
            self.capacity
        }
        
        fn get_mut(&mut self, index: usize) -> &mut (K, V) {
            &mut self.data[index]
        }
    }

    let mut indices = vec![];
    let mut entries = MockEntries::new(1);

    {
        let hasher = MockHasher;
        let mut ref_mut = RefMut::new(&mut indices, &mut entries);

        RawVacantEntryMut {
            map: ref_mut,
            hash_builder: &hasher,
        }
        .insert_hashed_nocheck(42, "key1", "value1");
        
        // This will exceed capacity and should panic
        RawVacantEntryMut {
            map: ref_mut,
            hash_builder: &hasher,
        }
        .insert_hashed_nocheck(43, "key2", "value2");
    }
}

