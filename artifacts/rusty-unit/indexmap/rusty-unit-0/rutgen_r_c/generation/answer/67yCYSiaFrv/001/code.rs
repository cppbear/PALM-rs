// Answer 0

#[test]
fn test_get_full_key_not_found() {
    struct MyMap {
        entries: Vec<Bucket<String, i32>>,
    }

    impl MyMap {
        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.entries
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            None // Simulating that the key is not found
        }
    }

    let my_map = MyMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: "key1".to_string(), value: 1 },
            Bucket { hash: HashValue::default(), key: "key2".to_string(), value: 2 },
        ],
    };

    let result: Option<(usize, &String, &i32)> = my_map.get_full("non_existent_key");
    assert_eq!(result, None);
}

#[test]
fn test_get_full_key_not_found_empty_map() {
    struct MyMap {
        entries: Vec<Bucket<String, i32>>,
    }

    impl MyMap {
        fn as_entries(&self) -> &[Bucket<String, i32>] {
            &self.entries
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            None // Simulating that the key is not found
        }
    }

    let my_map = MyMap { entries: vec![] };

    let result: Option<(usize, &String, &i32)> = my_map.get_full("key1");
    assert_eq!(result, None);
}

