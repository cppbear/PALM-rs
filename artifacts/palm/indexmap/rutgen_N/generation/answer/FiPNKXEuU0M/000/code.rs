// Answer 0

#[test]
fn test_sort_by_cached_key() {
    struct TestSet {
        entries: Vec<(i32, char)>,
    }

    impl TestSet {
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut Vec<(i32, char)>),
        {
            f(&mut self.entries);
        }

        fn sort_by_cached_key<K, F>(&mut self, mut sort_key: F)
        where
            K: Ord,
            F: FnMut(&(i32, char)) -> K,
        {
            self.with_entries(move |entries| {
                entries.sort_by_cached_key(move |a| sort_key(&a));
            });
        }
    }

    let mut test_set = TestSet {
        entries: vec![(3, 'c'), (1, 'a'), (2, 'b')],
    };

    test_set.sort_by_cached_key(|&(_, value)| value);

    assert_eq!(test_set.entries, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn test_sort_by_cached_key_with_identical_keys() {
    struct TestSet {
        entries: Vec<(i32, char)>,
    }

    impl TestSet {
        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut Vec<(i32, char)>),
        {
            f(&mut self.entries);
        }

        fn sort_by_cached_key<K, F>(&mut self, mut sort_key: F)
        where
            K: Ord,
            F: FnMut(&(i32, char)) -> K,
        {
            self.with_entries(move |entries| {
                entries.sort_by_cached_key(move |a| sort_key(&a));
            });
        }
    }

    let mut test_set = TestSet {
        entries: vec![(3, 'a'), (1, 'a'), (2, 'b')],
    };

    test_set.sort_by_cached_key(|&(_, value)| value);

    assert_eq!(test_set.entries, vec![(3, 'a'), (1, 'a'), (2, 'b')]);
}

