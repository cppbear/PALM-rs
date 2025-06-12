// Answer 0

#[test]
fn test_insert_hashed_nocheck() {
    struct HashValue(usize);
    
    struct TestMap<K, V> {
        map: std::collections::HashMap<HashValue, (K, V)>,
    }

    impl<K, V> TestMap<K, V> {
        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn insert_unique(&mut self, hash: HashValue, key: K, value: V) -> &mut (K, V) {
            self.map.insert(hash, (key, value));
            self.map.get_mut(&hash).unwrap()
        }

        fn into_muts(&mut self) -> (&mut K, &mut V) {
            let hash = self.map.keys().next().unwrap(); 
            let entry = self.map.get_mut(hash).unwrap();
            (&mut entry.0, &mut entry.1)
        }
    }

    let mut test_map: TestMap<String, i32> = TestMap::new();
    
    let key = String::from("test_key");
    let value = 42;
    let hash_value: u64 = 123;

    let (k_ref, v_ref) = test_map.insert_hashed_nocheck(hash_value, key.clone(), value);

    assert_eq!(*k_ref, key);
    assert_eq!(*v_ref, value);

    // Inserting another key-value
    let key2 = String::from("another_key");
    let value2 = 84;
    let hash_value2: u64 = 456;

    let (k2_ref, v2_ref) = test_map.insert_hashed_nocheck(hash_value2, key2.clone(), value2);

    assert_eq!(*k2_ref, key2);
    assert_eq!(*v2_ref, value2);

    // Check for correct values after insertion
    assert_eq!(*test_map.map.get(&HashValue(hash_value)).unwrap(), (key, value));
    assert_eq!(*test_map.map.get(&HashValue(hash_value2)).unwrap(), (key2, value2));
}

