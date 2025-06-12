// Answer 0

#[test]
fn test_insert_sorted_unique_value() {
    struct TestSet {
        map: indexmap::IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: indexmap::IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut test_set = TestSet::new();
    let (index, is_new) = test_set.insert_sorted(10);
    assert_eq!(index, 0);
    assert!(is_new);

    let (index, is_new) = test_set.insert_sorted(20);
    assert_eq!(index, 1);
    assert!(is_new);

    let (index, is_new) = test_set.insert_sorted(15);
    assert_eq!(index, 1);
    assert!(is_new);
}

#[test]
fn test_insert_sorted_existing_value() {
    struct TestSet {
        map: indexmap::IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: indexmap::IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut test_set = TestSet::new();
    let (index, is_new) = test_set.insert_sorted(10);
    assert_eq!(index, 0);
    assert!(is_new);

    let (index, is_new) = test_set.insert_sorted(10);
    assert_eq!(index, 0);
    assert!(!is_new);
}

#[test]
fn test_insert_sorted_multiple_values() {
    struct TestSet {
        map: indexmap::IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: indexmap::IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut test_set = TestSet::new();
    test_set.insert_sorted(30);
    test_set.insert_sorted(10);
    test_set.insert_sorted(20);
    test_set.insert_sorted(20); // existing value, should return false

    let (index, is_new) = test_set.insert_sorted(25);
    assert_eq!(index, 2);
    assert!(is_new);
}

#[test]
fn test_insert_sorted_boundary() {
    struct TestSet {
        map: indexmap::IndexMap<i32, ()>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                map: indexmap::IndexMap::new(),
            }
        }

        fn insert_sorted(&mut self, value: i32) -> (usize, bool) {
            let (index, existing) = self.map.insert_sorted(value, ());
            (index, existing.is_none())
        }
    }

    let mut test_set = TestSet::new();
    let (index, is_new) = test_set.insert_sorted(i32::MIN);
    assert_eq!(index, 0);
    assert!(is_new);

    let (index, is_new) = test_set.insert_sorted(i32::MAX);
    assert_eq!(index, 1);
    assert!(is_new);
}

