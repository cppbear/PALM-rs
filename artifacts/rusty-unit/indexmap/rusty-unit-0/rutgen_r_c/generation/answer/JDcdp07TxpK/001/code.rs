// Answer 0

#[test]
fn test_sorted_by_with_empty_set() {
    struct TestSet {
        entries: Vec<Bucket<i32>>,
    }

    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let set = TestSet { entries: vec![] };
    let _iter = set.sorted_by(|a, b| a.cmp(b));
}

#[test]
fn test_sorted_by_with_single_value() {
    struct TestSet {
        entries: Vec<Bucket<i32>>,
    }

    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let set = TestSet { entries: vec![Bucket { hash: 0, key: 42, value: () }] };
    let iter = set.sorted_by(|a, b| a.cmp(b));
    let collected: Vec<_> = iter.iter.collect();
    assert_eq!(collected, vec![42]);
}

#[test]
fn test_sorted_by_with_multiple_values() {
    struct TestSet {
        entries: Vec<Bucket<i32>>,
    }

    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let set = TestSet {
        entries: vec![
            Bucket { hash: 0, key: 3, value: () },
            Bucket { hash: 0, key: 1, value: () },
            Bucket { hash: 0, key: 2, value: () },
        ],
    };

    let iter = set.sorted_by(|a, b| a.cmp(b));
    let collected: Vec<_> = iter.iter.collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_sorted_by_with_reverse_order() {
    struct TestSet {
        entries: Vec<Bucket<i32>>,
    }

    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }

        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let set = TestSet {
        entries: vec![
            Bucket { hash: 0, key: 5, value: () },
            Bucket { hash: 0, key: 3, value: () },
            Bucket { hash: 0, key: 4, value: () },
        ],
    };

    let iter = set.sorted_by(|a, b| b.cmp(a)); // Reverse order
    let collected: Vec<_> = iter.iter.collect();
    assert_eq!(collected, vec![5, 4, 3]);
}

