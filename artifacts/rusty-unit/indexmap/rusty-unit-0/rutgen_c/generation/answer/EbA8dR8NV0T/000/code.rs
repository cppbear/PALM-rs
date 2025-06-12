// Answer 0

#[test]
fn test_sorted_unstable_by_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }

        fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<i32, i32>
        where
            F: FnMut(&i32, &i32, &i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap { entries: Vec::new() };
    let iter = map.sorted_unstable_by(|_k1, _v1, _k2, _v2| Ordering::Equal);
    assert_eq!(iter.as_slice(), &[]);
}

#[test]
fn test_sorted_unstable_by_single_entry() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }

        fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<i32, i32>
        where
            F: FnMut(&i32, &i32, &i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap {
        entries: vec![Bucket { hash: HashValue::new(1), key: 3, value: 30 }],
    };
    let iter = map.sorted_unstable_by(|_k1, _v1, _k2, _v2| Ordering::Equal);
    assert_eq!(iter.as_slice(), &[Bucket { hash: HashValue::new(1), key: 3, value: 30 }]);
}

#[test]
fn test_sorted_unstable_by_multiple_entries() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }

        fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<i32, i32>
        where
            F: FnMut(&i32, &i32, &i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 3, value: 30 },
            Bucket { hash: HashValue::new(2), key: 1, value: 10 },
            Bucket { hash: HashValue::new(3), key: 2, value: 20 },
        ],
    };

    let iter = map.sorted_unstable_by(|k1, _v1, k2, _v2| k1.cmp(k2));
    let sorted_entries: Vec<_> = iter.as_slice().to_vec();
    assert_eq!(sorted_entries, vec![
        Bucket { hash: HashValue::new(2), key: 1, value: 10 },
        Bucket { hash: HashValue::new(3), key: 2, value: 20 },
        Bucket { hash: HashValue::new(1), key: 3, value: 30 },
    ]);
}

