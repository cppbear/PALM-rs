// Answer 0

#[test]
fn test_insert_sorted_new_key() {
    struct TestMap {
        keys: Vec<i32>,
        values: Vec<String>,
    }

    impl TestMap {
        pub fn binary_search_keys(&self, key: &i32) -> Result<usize, usize> {
            self.keys.binary_search(key)
        }

        pub fn insert_before(&mut self, index: usize, key: i32, value: String) -> (usize, Option<String>) {
            self.keys.insert(index, key);
            self.values.insert(index, value.clone());
            (index, None)
        }

        pub fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            match self.binary_search_keys(&key) {
                Ok(i) => (i, Some(std::mem::replace(&mut self.values[i], value))),
                Err(i) => self.insert_before(i, key, value),
            }
        }
    }

    let mut map = TestMap {
        keys: vec![1, 3, 5, 7],
        values: vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
    };

    let (index, old_value) = map.insert_sorted(4, "new_value".to_string());
    assert_eq!(index, 3);
    assert_eq!(old_value, None);
    assert_eq!(map.keys, vec![1, 3, 4, 5, 7]);
    assert_eq!(map.values, vec!["a".to_string(), "b".to_string(), "new_value".to_string(), "c".to_string(), "d".to_string()]);
}

#[test]
fn test_insert_sorted_duplicate_key() {
    struct TestMap {
        keys: Vec<i32>,
        values: Vec<String>,
    }

    impl TestMap {
        pub fn binary_search_keys(&self, key: &i32) -> Result<usize, usize> {
            self.keys.binary_search(key)
        }

        pub fn insert_before(&mut self, index: usize, key: i32, value: String) -> (usize, Option<String>) {
            self.keys.insert(index, key);
            self.values.insert(index, value.clone());
            (index, None)
        }

        pub fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            match self.binary_search_keys(&key) {
                Ok(i) => (i, Some(std::mem::replace(&mut self.values[i], value))),
                Err(i) => self.insert_before(i, key, value),
            }
        }
    }

    let mut map = TestMap {
        keys: vec![2, 4, 6],
        values: vec!["x".to_string(), "y".to_string(), "z".to_string()],
    };

    let (index, old_value) = map.insert_sorted(4, "updated_value".to_string());
    assert_eq!(index, 1);
    assert_eq!(old_value, Some("y".to_string()));
    assert_eq!(map.keys, vec![2, 4, 6]);
    assert_eq!(map.values, vec!["x".to_string(), "updated_value".to_string(), "z".to_string()]);
}

#[test]
#[should_panic]
fn test_insert_sorted_when_not_sorted() {
    struct TestMap {
        keys: Vec<i32>,
        values: Vec<String>,
    }

    impl TestMap {
        pub fn binary_search_keys(&self, key: &i32) -> Result<usize, usize> {
            self.keys.binary_search(key)
        }

        pub fn insert_before(&mut self, index: usize, key: i32, value: String) -> (usize, Option<String>) {
            self.keys.insert(index, key);
            self.values.insert(index, value.clone());
            (index, None)
        }

        pub fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            match self.binary_search_keys(&key) {
                Ok(i) => (i, Some(std::mem::replace(&mut self.values[i], value))),
                Err(i) => self.insert_before(i, key, value),
            }
        }
    }

    let mut map = TestMap {
        keys: vec![3, 1, 2], // Unsorted keys
        values: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
    };

    let _ = map.insert_sorted(4, "new".to_string()); // This may panic if the binary search keys are not handled properly in an unsorted context
}

