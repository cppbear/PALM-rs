// Answer 0

#[test]
fn test_sorted_by_empty_map() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }
    
    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }
        
        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, i32>
        where
            F: FnMut(&i32, &i32, &i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap { entries: vec![] };
    let iter = map.sorted_by(|_, _, _, _| Ordering::Equal);
    assert!(iter.iter.len() == 0);
}

#[test]
fn test_sorted_by_single_entry() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }
    
    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }
        
        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, i32>
        where
            F: FnMut(&i32, &i32, &i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap {
        entries: vec![Bucket { hash: HashValue::default(), key: 1, value: 10 }],
    };
    let mut iter = map.sorted_by(|_, _, _, _| Ordering::Equal);
    assert_eq!(iter.iter.next(), Some(Bucket { hash: HashValue::default(), key: 1, value: 10 }));
}

#[test]
fn test_sorted_by_multiple_entries() {
    struct TestMap {
        entries: Vec<Bucket<i32, i32>>,
    }
    
    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32, i32>> {
            self.entries
        }
        
        fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32, i32>
        where
            F: FnMut(&i32, &i32, &i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
            IntoIter::new(entries)
        }
    }

    let map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 2, value: 20 },
            Bucket { hash: HashValue::default(), key: 1, value: 10 }
        ],
    };

    let mut iter = map.sorted_by(|a_key, a_value, b_key, b_value| {
        a_key.cmp(b_key) // Sort by key
    });
    
    assert_eq!(iter.iter.next(), Some(Bucket { hash: HashValue::default(), key: 1, value: 10 }));
    assert_eq!(iter.iter.next(), Some(Bucket { hash: HashValue::default(), key: 2, value: 20 }));
}

