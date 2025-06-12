// Answer 0

#[test]
fn test_build_hashes_inner() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct MyHasher {
        hasher: DefaultHasher,
    }

    impl MyHasher {
        fn new() -> Self {
            Self {
                hasher: DefaultHasher::new(),
            }
        }
        
        fn finish(self) -> u64 {
            self.hasher.finish()
        }
    }

    impl Hasher for MyHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.hasher.write(bytes);
        }

        fn finish(&self) -> u64 {
            self.hasher.finish()
        }
    }

    #[derive(Eq, PartialEq, Hash)]
    struct Key {
        value: u64,
    }

    struct Mock {
        hash_builder: MyHasher,
    }

    impl Mock {
        fn new() -> Self {
            Self {
                hash_builder: MyHasher::new(),
            }
        }

        fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> [u64; N]
        where
            Q: Hash + ?Sized,
        {
            let mut hashes = [0_u64; N];
            for i in 0..N {
                hashes[i] = self.hash(ks[i]);
            }
            hashes
        }

        fn hash<Q: Hash>(&self, value: &Q) -> u64 {
            let mut hasher = self.hash_builder.clone(); // Clone the hasher for each hash calculation
            value.hash(&mut hasher);
            hasher.finish()
        }
    }

    let mock = Mock::new();
    let keys: [&Key; 3] = [&Key { value: 1 }, &Key { value: 2 }, &Key { value: 3 }];
    let hashes = mock.build_hashes_inner(&keys);

    assert_eq!(hashes.len(), 3);
}

