// Answer 0

#[test]
fn test_capacity_empty_set() {
    struct TestSet {
        map: std::collections::HashMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: std::collections::HashMap::new(),
            }
        }

        fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let set = TestSet::new();
    assert_eq!(set.capacity(), 0);
}

#[test]
fn test_capacity_filled_set() {
    struct TestSet {
        map: std::collections::HashMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: std::collections::HashMap::new(),
            }
        }

        fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let mut set = TestSet::new();
    set.map.insert(1, ());
    set.map.insert(2, ());

    assert!(set.capacity() >= 2);
}

#[test]
fn test_capacity_after_reserve() {
    struct TestSet {
        map: std::collections::HashMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: std::collections::HashMap::new(),
            }
        }

        fn reserve(&mut self, additional: usize) {
            self.map.reserve(additional);
        }

        fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let mut set = TestSet::new();
    set.reserve(10);

    assert!(set.capacity() >= 10);
}

#[test]
fn test_capacity_increase_after_insertions() {
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

        fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let mut set = TestSet::new();
    for i in 0..15 {
        set.insert(i);
    }

    assert!(set.capacity() >= 15);
}

