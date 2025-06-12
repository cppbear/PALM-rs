// Answer 0

#[test]
fn test_insert_sorted_existing_key() {
    struct Map {
        keys: Vec<i32>,
        values: Vec<String>,
    }

    impl Map {
        fn binary_search_keys(&self, key: &i32) -> Result<usize, usize> {
            self.keys.binary_search(key)
        }

        fn insert_before(&mut self, index: usize, key: i32, value: String) -> (usize, Option<String>) {
            self.keys.insert(index, key);
            self.values.insert(index, value);
            (index, None)
        }

        fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            match self.binary_search_keys(&key) {
                Ok(i) => (i, Some(std::mem::replace(&mut self.values[i], value))),
                Err(i) => self.insert_before(i, key, value),
            }
        }
    }

    let mut map = Map {
        keys: vec![1, 2, 3],
        values: vec!["one".to_string(), "two".to_string(), "three".to_string()],
    };

    let (index, old_value) = map.insert_sorted(2, "UPDATED".to_string());

    assert_eq!(index, 1);
    assert_eq!(old_value, Some("two".to_string()));
    assert_eq!(map.values, vec!["one".to_string(), "UPDATED".to_string(), "three".to_string()]);
}

#[test]
fn test_insert_sorted_new_key() {
    struct Map {
        keys: Vec<i32>,
        values: Vec<String>,
    }

    impl Map {
        fn binary_search_keys(&self, key: &i32) -> Result<usize, usize> {
            self.keys.binary_search(key)
        }

        fn insert_before(&mut self, index: usize, key: i32, value: String) -> (usize, Option<String>) {
            self.keys.insert(index, key);
            self.values.insert(index, value);
            (index, None)
        }

        fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
            match self.binary_search_keys(&key) {
                Ok(i) => (i, Some(std::mem::replace(&mut self.values[i], value))),
                Err(i) => self.insert_before(i, key, value),
            }
        }
    }

    let mut map = Map {
        keys: vec![1, 2, 3],
        values: vec!["one".to_string(), "two".to_string(), "three".to_string()],
    };

    let (index, old_value) = map.insert_sorted(4, "four".to_string());

    assert_eq!(index, 3);
    assert_eq!(old_value, None);
    assert_eq!(map.keys, vec![1, 2, 3, 4]);
    assert_eq!(map.values, vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string()]);
}

