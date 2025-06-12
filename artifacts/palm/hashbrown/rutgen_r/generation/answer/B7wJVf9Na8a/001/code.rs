// Answer 0

#[test]
fn test_insert_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{BuildHasher, Hash};

    struct HashMapWrapper<K, V> {
        table: HashMap<K, V, DefaultHasher>,
    }

    impl<K: Hash + Eq, V> HashMapWrapper<K, V> {
        fn new() -> Self {
            HashMapWrapper {
                table: HashMap::new(),
            }
        }

        fn insert_entry(
            &mut self,
            key: K,
            value: V,
        ) -> RawOccupiedEntryMut<K, V, DefaultHasher, std::alloc::Global> {
            let hash_builder = DefaultHasher::new();
            let hash = make_hash::<K, DefaultHasher>(&hash_builder, &key);
            let elem = self.table.insert(
                key,
                value,
                make_hasher::<_, V, DefaultHasher>(&hash_builder),
            );
            RawOccupiedEntryMut {
                elem,
                table: &self.table,
                hash_builder,
            }
        }
    }

    struct RawOccupiedEntryMut<'a, K, V, S: BuildHasher, A> {
        elem: Option<(K, V)>,
        table: &'a HashMap<K, V, S>,
        hash_builder: S,
    }

    fn make_hash<K: Hash, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
        let mut hasher = hash_builder.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn make_hasher<K, V, S: BuildHasher>(_hash_builder: &S) -> fn(K, V) -> (K, V) {
        |k: K, v: V| (k, v)
    }

    let mut map = HashMapWrapper::new();
    let key = "test_key";
    let value = "test_value";

    let entry = map.insert_entry(key, value);
    assert!(entry.elem.is_some());
    assert_eq!(entry.elem.unwrap(), (key, value));
}

