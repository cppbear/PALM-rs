// Answer 0

#[test]
fn test_shrink_to_min_capacity_with_valid_value() {
    use indexmap::IndexMap;

    struct TestSet {
        map: IndexMap<u32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: u32) {
            self.map.insert(key, ());
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.map.shrink_to(min_capacity);
        }

        fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let mut set = TestSet::new();
    for i in 0..10 {
        set.insert(i);
    }

    set.shrink_to(5);
    assert!(set.capacity() >= 5);
}

#[test]
fn test_shrink_to_min_capacity_exceeding_current_capacity() {
    use indexmap::IndexMap;

    struct TestSet {
        map: IndexMap<u32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: u32) {
            self.map.insert(key, ());
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.map.shrink_to(min_capacity);
        }

        fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let mut set = TestSet::new();
    for i in 0..3 {
        set.insert(i);
    }

    let initial_capacity = set.capacity();
    set.shrink_to(initial_capacity + 5);
    assert!(set.capacity() >= initial_capacity);
}

#[test]
#[should_panic]
fn test_shrink_to_zero_capacity_negative() {
    use indexmap::IndexMap;

    struct TestSet {
        map: IndexMap<u32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, key: u32) {
            self.map.insert(key, ());
        }

        fn shrink_to(&mut self, min_capacity: usize) {
            self.map.shrink_to(min_capacity);
        }
    }

    let mut set = TestSet::new();
    set.shrink_to(usize::MAX);
}

