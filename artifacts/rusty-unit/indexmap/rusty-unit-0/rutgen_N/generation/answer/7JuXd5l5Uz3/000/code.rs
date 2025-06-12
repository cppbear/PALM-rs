// Answer 0

#[test]
fn test_len_empty_map() {
    struct Map {
        core: std::collections::HashMap<i32, i32>,
    }

    impl Map {
        pub fn new() -> Self {
            Map {
                core: std::collections::HashMap::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.core.len()
        }
    }

    let map = Map::new();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_len_non_empty_map() {
    struct Map {
        core: std::collections::HashMap<i32, i32>,
    }

    impl Map {
        pub fn new() -> Self {
            Map {
                core: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: i32, value: i32) {
            self.core.insert(key, value);
        }

        pub fn len(&self) -> usize {
            self.core.len()
        }
    }

    let mut map = Map::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.len(), 2);
}

#[test]
fn test_len_after_removal() {
    struct Map {
        core: std::collections::HashMap<i32, i32>,
    }

    impl Map {
        pub fn new() -> Self {
            Map {
                core: std::collections::HashMap::new(),
            }
        }

        pub fn insert(&mut self, key: i32, value: i32) {
            self.core.insert(key, value);
        }

        pub fn remove(&mut self, key: &i32) {
            self.core.remove(key);
        }

        pub fn len(&self) -> usize {
            self.core.len()
        }
    }

    let mut map = Map::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.remove(&1);
    assert_eq!(map.len(), 1);
}

