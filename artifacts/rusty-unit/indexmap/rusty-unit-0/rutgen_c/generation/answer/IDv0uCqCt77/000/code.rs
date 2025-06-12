// Answer 0

#[test]
fn test_sorted_unstable_by() {
    struct TestIndexSet {
        data: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(data: Vec<i32>) -> Self {
            TestIndexSet { data }
        }

        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.data.into_iter().map(|key| Bucket { hash: 0, key, value: () }).collect()
        }

        fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let set = TestIndexSet::new(vec![3, 1, 2]);

    let sorted_iter = set.sorted_unstable_by(|a, b| a.cmp(b));
    let sorted_values: Vec<i32> = sorted_iter.iter.collect();

    assert_eq!(sorted_values, vec![1, 2, 3]);
}

#[test]
fn test_sorted_unstable_by_empty() {
    struct TestIndexSet {
        data: Vec<i32>,
    }

    impl TestIndexSet {
        fn new(data: Vec<i32>) -> Self {
            TestIndexSet { data }
        }

        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.data.into_iter().map(|key| Bucket { hash: 0, key, value: () }).collect()
        }

        fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let set = TestIndexSet::new(vec![]);

    let sorted_iter = set.sorted_unstable_by(|a, b| a.cmp(b));
    let sorted_values: Vec<i32> = sorted_iter.iter.collect();

    assert_eq!(sorted_values, vec![]);
}

