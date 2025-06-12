// Answer 0

#[test]
fn test_sort_unstable_empty_set() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: Vec::new() }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }
    }

    let mut set = TestSet::new();
    set.sort_unstable();
    assert!(set.map.is_empty());
}

#[test]
fn test_sort_unstable_single_element() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![5] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }
    }

    let mut set = TestSet::new();
    set.sort_unstable();
    assert_eq!(set.map, vec![5]);
}

#[test]
fn test_sort_unstable_sorted_elements() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![1, 2, 3, 4, 5] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }
    }

    let mut set = TestSet::new();
    set.sort_unstable();
    assert_eq!(set.map, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_sort_unstable_reverse_sorted_elements() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![5, 4, 3, 2, 1] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }
    }

    let mut set = TestSet::new();
    set.sort_unstable();
    assert_eq!(set.map, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_sort_unstable_unsorted_elements() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![3, 1, 4, 2, 5] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }
    }

    let mut set = TestSet::new();
    set.sort_unstable();
    assert_eq!(set.map, vec![1, 2, 3, 4, 5]);
}

