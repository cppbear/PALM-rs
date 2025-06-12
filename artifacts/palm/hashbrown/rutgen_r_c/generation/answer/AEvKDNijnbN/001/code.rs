// Answer 0

#[test]
fn test_build_hashes_inner_with_valid_keys() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct CustomHasher {
        hasher: DefaultHasher,
    }

    impl BuildHasher for CustomHasher {
        fn build_hasher(&self) -> Self {
            CustomHasher {
                hasher: DefaultHasher::new(),
            }
        }
    }

    struct TestMap<K, V, S = CustomHasher> {
        hash_builder: S,
        _marker: PhantomData<(K, V)>,
    }

    impl<K, V, S> TestMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        fn new(hash_builder: S) -> Self {
            TestMap {
                hash_builder,
                _marker: PhantomData,
            }
        }

        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + Equivalent<K> + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = make_hash::<Q, S>(&self.hash_builder, ks[i]);
            }
            hashes
        }
    }

    let map = TestMap::new(CustomHasher { hasher: DefaultHasher::new() });
    
    let keys = ["key1", "key2", "key3"];
    let hashes = map.build_hashes_inner([&keys[0], &keys[1], &keys[2]]);
    
    assert_eq!(hashes.len(), 3);
}

#[test]
#[should_panic]
fn test_build_hashes_inner_with_empty_keys() {
    struct CustomHasher {
        hasher: DefaultHasher,
    }

    impl BuildHasher for CustomHasher {
        fn build_hasher(&self) -> Self {
            CustomHasher {
                hasher: DefaultHasher::new(),
            }
        }
    }

    struct TestMap<K, V, S = CustomHasher> {
        hash_builder: S,
        _marker: PhantomData<(K, V)>,
    }

    impl<K, V, S> TestMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        fn new(hash_builder: S) -> Self {
            TestMap {
                hash_builder,
                _marker: PhantomData,
            }
        }

        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + Equivalent<K> + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = make_hash::<Q, S>(&self.hash_builder, ks[i]);
            }
            hashes
        }
    }

    let map = TestMap::new(CustomHasher { hasher: DefaultHasher::new() });

    // Attempting to call with zero-length array should panic
    let hashes: [u64; 0] = map.build_hashes_inner([]);
}

#[test]
fn test_build_hashes_inner_with_different_key_counts() {
    struct CustomHasher {
        hasher: DefaultHasher,
    }

    impl BuildHasher for CustomHasher {
        fn build_hasher(&self) -> Self {
            CustomHasher {
                hasher: DefaultHasher::new(),
            }
        }
    }

    struct TestMap<K, V, S = CustomHasher> {
        hash_builder: S,
        _marker: PhantomData<(K, V)>,
    }

    impl<K, V, S> TestMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        fn new(hash_builder: S) -> Self {
            TestMap {
                hash_builder,
                _marker: PhantomData,
            }
        }

        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + Equivalent<K> + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = make_hash::<Q, S>(&self.hash_builder, ks[i]);
            }
            hashes
        }
    }

    let map = TestMap::new(CustomHasher { hasher: DefaultHasher::new() });
    
    let keys_2 = ["key1", "key2"];
    let hashes_2 = map.build_hashes_inner([&keys_2[0], &keys_2[1]]);
    
    assert_eq!(hashes_2.len(), 2);

    let keys_4 = ["key1", "key2", "key3", "key4"];
    let hashes_4 = map.build_hashes_inner([&keys_4[0], &keys_4[1], &keys_4[2], &keys_4[3]]);
    
    assert_eq!(hashes_4.len(), 4);
}

