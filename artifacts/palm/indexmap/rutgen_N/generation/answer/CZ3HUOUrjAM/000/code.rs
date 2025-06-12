// Answer 0

#[test]
fn test_retain2_with_keep_function() {
    struct Map<K, V> {
        core: std::collections::HashMap<K, V>
    }

    impl<K, V> Map<K, V> {
        fn retain_in_order<F>(&mut self, mut keep: F)
        where
            F: FnMut(&mut K, &mut V) -> bool,
        {
            let keys_to_remove: Vec<_> = self.core.iter_mut()
                .filter(|(k, v)| !keep(k, v))
                .map(|(k, _)| k.clone())
                .collect();

            for k in keys_to_remove {
                self.core.remove(&k);
            }
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut K, &mut V) -> bool,
        {
            self.retain_in_order(keep);
        }
    }

    let mut map = Map {
        core: vec![(1, "one"), (2, "two"), (3, "three")].into_iter().collect()
    };

    map.retain2(|_k, v| *v != "two");
    
    assert_eq!(map.core.len(), 2);
    assert!(map.core.contains_key(&1));
    assert!(map.core.contains_key(&3));
    assert!(!map.core.contains_key(&2));
}

#[test]
fn test_retain2_empty_map() {
    struct Map<K, V> {
        core: std::collections::HashMap<K, V>
    }

    impl<K, V> Map<K, V> {
        fn retain_in_order<F>(&mut self, mut keep: F)
        where
            F: FnMut(&mut K, &mut V) -> bool,
        {
            let keys_to_remove: Vec<_> = self.core.iter_mut()
                .filter(|(k, v)| !keep(k, v))
                .map(|(k, _)| k.clone())
                .collect();

            for k in keys_to_remove {
                self.core.remove(&k);
            }
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut K, &mut V) -> bool,
        {
            self.retain_in_order(keep);
        }
    }

    let mut map: Map<i32, &str> = Map {
        core: std::collections::HashMap::new()
    };

    map.retain2(|_k, _v| false);
    
    assert_eq!(map.core.len(), 0);
}

