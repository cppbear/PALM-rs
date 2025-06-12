// Answer 0

#[test]
fn test_get_many_unchecked_mut_inner() {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    struct DummyKey {
        value: i32,
    }

    impl Hash for DummyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    struct DummyValue {
        data: i32,
    }

    struct HashMap<K, V> {
        table: Vec<Option<(K, V)>>,
    }

    impl<K, V> HashMap<K, V> {
        fn new() -> Self {
            Self { table: Vec::new() }
        }

        // Simulating the table's get_many_unchecked_mut method for testing purposes
        fn get_many_unchecked_mut<Q>(
            &mut self,
            hashes: Vec<usize>,
            equivalent: impl Fn(usize, &(K, V)) -> bool,
        ) -> Vec<Option<&mut (K, V)>> {
            hashes.iter().map(|&i| {
                if i < self.table.len() && equivalent(i, self.table[i].as_ref().unwrap()) {
                    self.table[i].as_mut()
                } else {
                    None
                }
            }).collect()
        }

        fn build_hashes_inner<Q>(&self, ks: [&Q; 2]) -> Vec<usize> {
            ks.iter()
                .map(|key| {
                    // Just use the value of the key as a hash for this simple test case
                    key.value as usize % self.table.len()
                })
                .collect()
        }
    }

    unsafe {
        let mut map = HashMap::<DummyKey, DummyValue>::new();
        map.table.push(Some((DummyKey { value: 1 }, DummyValue { data: 10 })));
        map.table.push(Some((DummyKey { value: 2 }, DummyValue { data: 20 })));

        let k1 = DummyKey { value: 1 };
        let k2 = DummyKey { value: 2 };

        let result: Vec<_> = map.get_many_unchecked_mut_inner(&[&k1, &k2]);

        assert_eq!(result.len(), 2);
        assert!(result[0].is_some());
        assert!(result[1].is_some());
        assert_eq!(result[0].unwrap().0.value, 1);
        assert_eq!(result[1].unwrap().0.value, 2);
    }
}

