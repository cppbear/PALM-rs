// Answer 0

#[test]
fn test_is_empty_true() {
    struct TestMap {
        data: std::collections::HashMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: std::collections::HashMap::new(),
            }
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let map = TestMap::new();
    assert!(map.is_empty());
}

#[test]
fn test_is_empty_false() {
    struct TestMap {
        data: std::collections::HashMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                data: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: i32) {
            self.data.insert(key, value);
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 10);
    assert!(!map.is_empty());
}

