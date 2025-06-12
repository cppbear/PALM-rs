// Answer 0

#[test]
fn test_shift_remove_full_existing_value() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    struct TestSet {
        map: IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn shift_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            self.map.shift_remove_full(value).map(|(i, x, ())| (i, x))
        }
    }

    let mut set = TestSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert_eq!(set.shift_remove_full(&2), Some((1, 2)));
    assert_eq!(set.shift_remove_full(&1), Some((0, 1)));
    assert_eq!(set.shift_remove_full(&3), Some((0, 3))); // After removing 1 and 2, 3 is now at index 0
}

#[test]
fn test_shift_remove_full_non_existing_value() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    struct TestSet {
        map: IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn shift_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            self.map.shift_remove_full(value).map(|(i, x, ())| (i, x))
        }
    }

    let mut set = TestSet::new();
    set.insert(1);
    set.insert(2);
    
    assert_eq!(set.shift_remove_full(&3), None);
}

#[test]
fn test_shift_remove_full_empty_set() {
    use indexmap::IndexMap;
    use std::hash::Hash;

    struct TestSet {
        map: IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: IndexMap::new(),
            }
        }

        fn shift_remove_full(&mut self, value: &i32) -> Option<(usize, i32)> {
            self.map.shift_remove_full(value).map(|(i, x, ())| (i, x))
        }
    }

    let mut set = TestSet::new();
    
    assert_eq!(set.shift_remove_full(&1), None);
}

