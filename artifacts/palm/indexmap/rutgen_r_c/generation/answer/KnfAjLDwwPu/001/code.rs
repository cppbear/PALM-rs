// Answer 0

#[test]
fn test_get_key_value_not_present() {
    struct TestMap {
        data: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            self.data.iter().position(|(k, _)| k == key)
        }

        fn as_entries(&self) -> &[(i32, i32)] {
            &self.data
        }

        fn get_key_value<Q>(&self, key: &Q) -> Option<(&i32, &i32)>
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }
    }

    let map = TestMap::new();
    let key_to_test = 42; // key not present

    let result = map.get_key_value(&key_to_test);
    assert_eq!(result, None);
}

