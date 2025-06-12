// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    struct MockHashBuilder;

    struct MockEntry {
        key: usize,
        value: usize,
    }

    struct MockCore {
        entries: Vec<MockEntry>,
        indices: MockIndices,
    }

    struct MockIndices {
        hashes: Vec<u64>,
        indices: Vec<usize>,
    }

    impl MockIndices {
        fn find_entry<F>(&self, hash: u64, eq: F) -> Result<usize, &Self>
        where
            F: FnMut(&usize) -> bool,
        {
            for (i, &h) in self.hashes.iter().enumerate() {
                if h == hash && eq(&self.indices[i]) {
                    return Ok(self.indices[i]);
                }
            }
            Err(self)
        }
    }

    struct MockMap {
        core: MockCore,
        hash_builder: MockHashBuilder,
    }

    struct RawOccupiedEntryMut<'a, K, V> {
        entries: &'a mut Vec<MockEntry>,
        index: usize,
        hash_builder: std::marker::PhantomData<(K, V)>,
    }

    enum RawEntryMut<'a, K, V> {
        Occupied(RawOccupiedEntryMut<'a, K, V>),
        Vacant,
    }

    struct FromHashWrapper<'a, K, V> {
        map: &'a mut MockMap,
    }

    impl<'a, K, V> FromHashWrapper<'a, K, V> {
        pub fn from_hash<F>(self, hash: u64, mut is_match: F) -> RawEntryMut<'a, K, V>
        where
            F: FnMut(&K) -> bool,
        {
            let ref_entries = &mut self.map.core.entries;
            let eq = move |&i: &usize| is_match(&ref_entries[i].key);
            match self.map.core.indices.find_entry(hash, eq) {
                Ok(index) => RawEntryMut::Occupied(RawOccupiedEntryMut {
                    entries: ref_entries,
                    index,
                    hash_builder: std::marker::PhantomData,
                }),
                Err(_) => RawEntryMut::Vacant,
            }
        }
    }

    let mut entries = vec![MockEntry { key: 1, value: 10 }];
    let indices = MockIndices {
        hashes: vec![123],
        indices: vec![0],
    };
    let core = MockCore { entries, indices };
    let mut mock_map = MockMap {
        core,
        hash_builder: MockHashBuilder,
    };
    
    let wrapper = FromHashWrapper { map: &mut mock_map };
    let hash = 123;
    
    if let RawEntryMut::Occupied(entry) = wrapper.from_hash(hash, |key| *key == 1) {
        assert_eq!(entry.index, 0);
        assert_eq!(entry.entries[0].key, 1);
        assert_eq!(entry.entries[0].value, 10);
    } else {
        panic!("Expected an occupied entry but got vacant.");
    }
}

