// Answer 0

#[test]
fn test_get_full_mut_existing_key() {
    struct TestHashMap {
        entries: Vec<Bucket<String, i32>>,
    }

    impl MutableKeys for TestHashMap {
        type Key = String;
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            for (i, bucket) in self.entries.iter_mut().enumerate() {
                if bucket.key == key {
                    return Some((i, &mut bucket.key, &mut bucket.value));
                }
            }
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            if index < self.entries.len() {
                let bucket = &mut self.entries[index];
                Some((&mut bucket.key, &mut bucket.value))
            } else {
                None
            }
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool,
        {
            unimplemented!()
        }
    }

    let mut map = TestHashMap {
        entries: vec![
            Bucket { hash: 1, key: "key1".to_string(), value: 10 },
            Bucket { hash: 2, key: "key2".to_string(), value: 20 },
        ],
    };

    let result = map.get_full_mut2(&"key1".to_string());
    assert!(result.is_some());
    let (index, key, value) = result.unwrap();
    assert_eq!(index, 0);
    assert_eq!(key, &"key1".to_string());
    assert_eq!(*value, 10);
}

#[test]
fn test_get_full_mut_non_existing_key() {
    struct TestHashMap {
        entries: Vec<Bucket<String, i32>>,
    }

    impl MutableKeys for TestHashMap {
        type Key = String;
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            for (i, bucket) in self.entries.iter_mut().enumerate() {
                if bucket.key == key {
                    return Some((i, &mut bucket.key, &mut bucket.value));
                }
            }
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            if index < self.entries.len() {
                let bucket = &mut self.entries[index];
                Some((&mut bucket.key, &mut bucket.value))
            } else {
                None
            }
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool,
        {
            unimplemented!()
        }
    }

    let mut map = TestHashMap {
        entries: vec![
            Bucket { hash: 1, key: "key1".to_string(), value: 10 },
            Bucket { hash: 2, key: "key2".to_string(), value: 20 },
        ],
    };

    let result = map.get_full_mut2(&"key3".to_string());
    assert!(result.is_none());
}

