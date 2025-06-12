// Answer 0

#[test]
fn test_sorted_unstable_by_with_sorted_values() {
    struct TestSet {
        values: Vec<i32>,
    }
    
    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.values.into_iter().map(|key| Bucket { hash: 0, key, value: () }).collect()
        }
        
        fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }
    
    let test_set = TestSet { values: vec![5, 3, 8, 1, 4] };
    let mut iter = test_set.sorted_unstable_by(|a, b| a.cmp(b));

    let sorted_values: Vec<i32> = iter.iter.collect();
    assert_eq!(sorted_values, vec![1, 3, 4, 5, 8]);
}

#[test]
fn test_sorted_unstable_by_empty_set() {
    struct TestSet {
        values: Vec<i32>,
    }
    
    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.values.into_iter().map(|key| Bucket { hash: 0, key, value: () }).collect()
        }
        
        fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }
    
    let test_set = TestSet { values: vec![] };
    let mut iter = test_set.sorted_unstable_by(|a, b| a.cmp(b));

    let sorted_values: Vec<i32> = iter.iter.collect();
    assert_eq!(sorted_values, vec![]);
} 

#[test]
fn test_sorted_unstable_by_reverse_order() {
    struct TestSet {
        values: Vec<i32>,
    }
    
    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.values.into_iter().map(|key| Bucket { hash: 0, key, value: () }).collect()
        }
        
        fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }
    
    let test_set = TestSet { values: vec![10, 20, 30, 15] };
    let mut iter = test_set.sorted_unstable_by(|a, b| b.cmp(a));

    let sorted_values: Vec<i32> = iter.iter.collect();
    assert_eq!(sorted_values, vec![30, 20, 15, 10]);
} 

#[test]
#[should_panic]
fn test_sorted_unstable_by_panic_on_invalid_comparison() {
    struct TestSet {
        values: Vec<i32>,
    }
    
    impl TestSet {
        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.values.into_iter().map(|key| Bucket { hash: 0, key, value: () }).collect()
        }
        
        fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }
    
    let test_set = TestSet { values: vec![1, 2, 3] };
    let _ = test_set.sorted_unstable_by(|a, b| {
        if *a == 2 && *b == 1 {
            panic!("Invalid comparison")
        } else {
            a.cmp(b)
        }
    });
} 


