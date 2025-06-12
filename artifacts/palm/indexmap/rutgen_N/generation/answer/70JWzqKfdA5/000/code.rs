// Answer 0

#[test]
fn test_pop_from_non_empty_set() {
    struct TestSet {
        map: std::collections::HashMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn pop(&mut self) -> Option<i32> {
            self.map.pop().map(|(x, ())| x)
        }
    }

    let mut set = TestSet::new();
    set.insert(1);
    set.insert(2);
    
    assert_eq!(set.pop(), Some(2));
    assert_eq!(set.pop(), Some(1));
    assert_eq!(set.pop(), None);
}

#[test]
fn test_pop_from_empty_set() {
    struct TestSet {
        map: std::collections::HashMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: std::collections::HashMap::new(),
            }
        }

        fn pop(&mut self) -> Option<i32> {
            self.map.pop().map(|(x, ())| x)
        }
    }

    let mut set = TestSet::new();
    
    assert_eq!(set.pop(), None);
}

