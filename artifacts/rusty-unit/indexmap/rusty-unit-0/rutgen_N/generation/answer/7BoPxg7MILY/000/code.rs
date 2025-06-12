// Answer 0

#[test]
fn test_insert_new_key() {
    struct Map {
        core: std::collections::HashMap<i32, i32>,
        order: Vec<i32>,
    }

    impl Map {
        fn new() -> Self {
            Self {
                core: std::collections::HashMap::new(),
                order: Vec::new(),
            }
        }

        fn hash(&self, key: &i32) -> usize {
            (*key % 10) as usize // Simple hash for illustration
        }

        fn insert_full(&mut self, key: i32, value: i32) -> (usize, Option<i32>) {
            let hash = self.hash(&key);
            if self.core.contains_key(&key) {
                let old_value = self.core.insert(key, value);
                (hash, old_value)
            } else {
                self.order.push(key);
                self.core.insert(key, value);
                (hash, None)
            }
        }
    }

    let mut map = Map::new();
    let (index, old_value) = map.insert_full(1, 10);
    assert_eq!(index, 1); // hash for key 1 is 1
    assert_eq!(old_value, None);

    let (index2, old_value2) = map.insert_full(1, 20);
    assert_eq!(index2, 1); // hash remains the same
    assert_eq!(old_value2, Some(10)); // Should return old value
}

#[test]
fn test_insert_existing_key() {
    struct Map {
        core: std::collections::HashMap<i32, i32>,
        order: Vec<i32>,
    }

    impl Map {
        fn new() -> Self {
            Self {
                core: std::collections::HashMap::new(),
                order: Vec::new(),
            }
        }

        fn hash(&self, key: &i32) -> usize {
            (*key % 10) as usize // Simple hash for illustration
        }

        fn insert_full(&mut self, key: i32, value: i32) -> (usize, Option<i32>) {
            let hash = self.hash(&key);
            if self.core.contains_key(&key) {
                let old_value = self.core.insert(key, value);
                (hash, old_value)
            } else {
                self.order.push(key);
                self.core.insert(key, value);
                (hash, None)
            }
        }
    }

    let mut map = Map::new();
    map.insert_full(2, 30);
    let (index, old_value) = map.insert_full(2, 40);
    assert_eq!(index, 2); // hash for key 2 is 2
    assert_eq!(old_value, Some(30)); // Should return old value
}

#[test]
fn test_insert_edge_case() {
    struct Map {
        core: std::collections::HashMap<i32, i32>,
        order: Vec<i32>,
    }

    impl Map {
        fn new() -> Self {
            Self {
                core: std::collections::HashMap::new(),
                order: Vec::new(),
            }
        }

        fn hash(&self, key: &i32) -> usize {
            (*key % 10) as usize // Simple hash for illustration
        }

        fn insert_full(&mut self, key: i32, value: i32) -> (usize, Option<i32>) {
            let hash = self.hash(&key);
            if self.core.contains_key(&key) {
                let old_value = self.core.insert(key, value);
                (hash, old_value)
            } else {
                self.order.push(key);
                self.core.insert(key, value);
                (hash, None)
            }
        }
    }

    let mut map = Map::new();
    let (index, old_value) = map.insert_full(0, 50);
    assert_eq!(index, 0); // hash for key 0 is 0
    assert_eq!(old_value, None); // Newly inserted

    let (index2, old_value2) = map.insert_full(0, 60);
    assert_eq!(index2, 0); // hash remains the same
    assert_eq!(old_value2, Some(50)); // Should return old value
}

