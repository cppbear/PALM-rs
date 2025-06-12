// Answer 0

#[test]
fn test_sort_by_cached_key() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            Self { values: vec![3, 1, 2] }
        }

        fn sort_by_cached_key<F>(&mut self, mut sort_key: F)
        where
            F: FnMut(&i32) -> i32,
        {
            let mut entries: Vec<&i32> = self.values.iter().collect();
            entries.sort_by_cached_key(|&a| sort_key(a));
            self.values = entries.into_iter().map(|&v| *v).collect();
        }
    }

    let mut test_set = TestSet::new();
    test_set.sort_by_cached_key(|&x| x);
    assert_eq!(test_set.values, vec![1, 2, 3]);
}

#[test]
fn test_sort_by_cached_key_empty() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            Self { values: vec![] }
        }

        fn sort_by_cached_key<F>(&mut self, mut sort_key: F)
        where
            F: FnMut(&i32) -> i32,
        {
            let mut entries: Vec<&i32> = self.values.iter().collect();
            entries.sort_by_cached_key(|&a| sort_key(a));
            self.values = entries.into_iter().map(|&v| *v).collect();
        }
    }

    let mut test_set = TestSet::new();
    test_set.sort_by_cached_key(|&x| x);
    assert_eq!(test_set.values, vec![]);
}

#[test]
fn test_sort_by_cached_key_stable_sort() {
    struct TestSet {
        values: Vec<(i32, i32)>,
    }

    impl TestSet {
        fn new() -> Self {
            Self { values: vec![(1, 2), (1, 1), (2, 3)] }
        }

        fn sort_by_cached_key<F>(&mut self, mut sort_key: F)
        where
            F: FnMut(&(i32, i32)) -> i32,
        {
            let mut entries: Vec<&(i32, i32)> = self.values.iter().collect();
            entries.sort_by_cached_key(|&a| sort_key(a));
            self.values = entries.into_iter().map(|&v| *v).collect();
        }
    }

    let mut test_set = TestSet::new();
    test_set.sort_by_cached_key(|&(x, _)| x);
    assert_eq!(test_set.values, vec![(1, 2), (1, 1), (2, 3)]);
}

