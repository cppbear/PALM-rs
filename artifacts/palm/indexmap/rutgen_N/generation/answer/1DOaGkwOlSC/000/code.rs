// Answer 0

#[test]
fn test_get_index_of_found() {
    struct TestStruct {
        entries: Vec<String>,
        indices: std::collections::HashMap<u64, usize>,
    }

    impl TestStruct {
        fn get_index_of<Q>(&self, hash: u64, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Equivalent<String>,
        {
            let eq = equivalent(key, &self.entries);
            self.indices.get(&hash).copied()
        }
    }

    struct EquivalentKey<'a>(&'a str);
    
    impl Equivalent<String> for EquivalentKey<'_> {
        fn equivalent(&self, other: &String) -> bool {
            self.0 == other.as_str()
        }
    }

    fn equivalent<Q: ?Sized, K>(key: &Q, entries: &Vec<K>) -> bool
    where
        Q: Equivalent<K>,
        K: std::fmt::Debug,
    {
        entries.iter().any(|entry| key.equivalent(entry))
    }

    let entries = vec![String::from("key1"), String::from("key2")];
    let indices = [(12345, 0), (67890, 1)].iter().cloned().collect();
    let test_struct = TestStruct { entries, indices };

    let hash_value = 12345;
    let key = EquivalentKey("key1");
    let index = test_struct.get_index_of(hash_value, &key);

    assert_eq!(index, Some(0));
}

#[test]
fn test_get_index_of_not_found() {
    struct TestStruct {
        entries: Vec<String>,
        indices: std::collections::HashMap<u64, usize>,
    }

    impl TestStruct {
        fn get_index_of<Q>(&self, hash: u64, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Equivalent<String>,
        {
            let eq = equivalent(key, &self.entries);
            self.indices.get(&hash).copied()
        }
    }

    struct EquivalentKey<'a>(&'a str);
    
    impl Equivalent<String> for EquivalentKey<'_> {
        fn equivalent(&self, other: &String) -> bool {
            self.0 == other.as_str()
        }
    }

    fn equivalent<Q: ?Sized, K>(key: &Q, entries: &Vec<K>) -> bool
    where
        Q: Equivalent<K>,
        K: std::fmt::Debug,
    {
        entries.iter().any(|entry| key.equivalent(entry))
    }

    let entries = vec![String::from("key1"), String::from("key2")];
    let indices = [(12345, 0)].iter().cloned().collect();
    let test_struct = TestStruct { entries, indices };

    let hash_value = 67890;
    let key = EquivalentKey("key3");
    let index = test_struct.get_index_of(hash_value, &key);

    assert_eq!(index, None);
}

