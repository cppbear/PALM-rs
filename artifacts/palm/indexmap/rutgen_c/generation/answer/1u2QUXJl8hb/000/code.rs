// Answer 0

#[test]
fn test_reverse_empty_map() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn reverse(&mut self) {
            self.core.reverse();
        }

        fn len(&self) -> usize {
            self.core.len()
        }
    }

    let mut map = TestMap::new();
    map.reverse();
    assert_eq!(map.len(), 0);
}

#[test]
fn test_reverse_single_element() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::with_capacity(1),
            }
        }

        fn insert(&mut self, key: i32, value: i32) {
            self.core.entries.push((key, value));
            self.core.indices.push(0); // Simplifying for test
        }

        fn reverse(&mut self) {
            self.core.reverse();
        }

        fn first(&self) -> Option<&(i32, i32)> {
            self.core.entries.first()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 100);
    map.reverse();
    assert_eq!(map.first(), Some(&(1, 100)));
}

#[test]
fn test_reverse_multiple_elements() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                core: IndexMapCore::new(),
            }
        }

        fn insert(&mut self, key: i32, value: i32) {
            self.core.entries.push((key, value));
            self.core.indices.push(self.core.len()); // Simplifying for test
        }

        fn reverse(&mut self) {
            self.core.reverse();
        }

        fn as_slice(&self) -> &[(i32, i32)] {
            &self.core.entries
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    map.reverse();
    
    assert_eq!(map.as_slice(), &[(3, 300), (2, 200), (1, 100)]);
}

