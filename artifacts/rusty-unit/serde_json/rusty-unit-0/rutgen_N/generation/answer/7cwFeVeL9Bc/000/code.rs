// Answer 0

#[test]
fn test_hash_preserve_order() {
    use std::hash::{Hasher, Hash};
    use std::collections::HashMap;
    use std::iter::FromIterator;

    struct MapWrapper<K, V> {
        map: HashMap<K, V>,
    }

    impl<K: Hash + Eq, V> MapWrapper<K, V> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            #[cfg(not(feature = "preserve_order"))]
            {
                self.map.hash(state);
            }

            #[cfg(feature = "preserve_order")]
            {
                let mut kv = Vec::from_iter(self.map.iter());
                kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
                kv.hash(state);
            }
        }
    }

    // Example test case with HashMap where preservation order feature is assumed to be enabled.
    let mut map = HashMap::new();
    map.insert("b", 2);
    map.insert("a", 1);

    let wrapper = MapWrapper { map };

    let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
    wrapper.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut map2 = HashMap::new();
    map2.insert("a", 1);
    map2.insert("b", 2);

    let wrapper2 = MapWrapper { map: map2 };

    let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
    wrapper2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

#[test]
#[cfg(not(feature = "preserve_order"))]
fn test_hash_no_preserve_order() {
    use std::hash::{Hasher, Hash};
    use std::collections::HashMap;

    struct MapWrapper<K, V> {
        map: HashMap<K, V>,
    }

    impl<K: Hash + Eq, V> MapWrapper<K, V> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            #[cfg(not(feature = "preserve_order"))]
            {
                self.map.hash(state);
            }

            #[cfg(feature = "preserve_order")]
            {
                let mut kv = Vec::from_iter(self.map.iter());
                kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
                kv.hash(state);
            }
        }
    }

    // Test case with unordered keys
    let mut map = HashMap::new();
    map.insert("b", 2);
    map.insert("a", 1);

    let wrapper = MapWrapper { map };

    let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
    wrapper.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    // Test case with same keys but different order
    let mut map2 = HashMap::new();
    map2.insert("a", 1);
    map2.insert("b", 2);

    let wrapper2 = MapWrapper { map: map2 };

    let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
    wrapper2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

