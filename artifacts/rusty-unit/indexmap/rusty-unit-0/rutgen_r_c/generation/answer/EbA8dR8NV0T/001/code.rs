// Answer 0

#[test]
fn test_sorted_unstable_by() {
    struct TestMap {
        data: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.data
        }

        fn sorted_unstable_by<F>(mut self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    // Test case: Normal case with integers and strings
    let map = TestMap {
        data: vec![
            Bucket { hash: HashValue(1), key: 3, value: "three".to_string() },
            Bucket { hash: HashValue(2), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue(3), key: 2, value: "two".to_string() },
        ],
    };

    let sorted_iter = map.sorted_unstable_by(|k1, v1, k2, v2| {
        if k1 == k2 {
            Ordering::Equal
        } else {
            k1.cmp(k2)
        }
    });

    let sorted_buckets: Vec<_> = sorted_iter.iter.collect();
    assert_eq!(sorted_buckets, vec![
        Bucket { hash: HashValue(2), key: 1, value: "one".to_string() },
        Bucket { hash: HashValue(3), key: 2, value: "two".to_string() },
        Bucket { hash: HashValue(1), key: 3, value: "three".to_string() },
    ]);
}

#[test]
fn test_sorted_unstable_by_empty() {
    struct TestMap {
        data: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.data
        }

        fn sorted_unstable_by<F>(mut self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    // Test case: Empty map
    let map = TestMap {
        data: Vec::new(),
    };

    let sorted_iter = map.sorted_unstable_by(|_, _, _, _| Ordering::Equal);
    let sorted_buckets: Vec<_> = sorted_iter.iter.collect();
    assert_eq!(sorted_buckets, Vec::<Bucket<i32, String>>::new());
}

#[test]
fn test_sorted_unstable_by_identical_keys() {
    struct TestMap {
        data: Vec<Bucket<i32, String>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, String>> {
            self.data
        }

        fn sorted_unstable_by<F>(mut self, mut cmp: F) -> IntoIter<i32, String>
        where
            F: FnMut(&i32, &String, &i32, &String) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    // Test case: Identical keys, different values
    let map = TestMap {
        data: vec![
            Bucket { hash: HashValue(1), key: 1, value: "a".to_string() },
            Bucket { hash: HashValue(2), key: 1, value: "b".to_string() },
            Bucket { hash: HashValue(3), key: 1, value: "c".to_string() },
        ],
    };

    let sorted_iter = map.sorted_unstable_by(|_, _, _, _| Ordering::Equal);
    let sorted_buckets: Vec<_> = sorted_iter.iter.collect();
    assert_eq!(sorted_buckets, vec![
        Bucket { hash: HashValue(1), key: 1, value: "a".to_string() },
        Bucket { hash: HashValue(2), key: 1, value: "b".to_string() },
        Bucket { hash: HashValue(3), key: 1, value: "c".to_string() },
    ]);
}

