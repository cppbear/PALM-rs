// Answer 0

#[test]
fn test_sorted_by() {
    struct IndexSet {
        entries: Vec<Bucket<i32, ()>>,
    }
    
    impl IndexSet {
        fn into_entries(self) -> Vec<Bucket<i32, ()>> {
            self.entries
        }
        
        fn new() -> Self {
            IndexSet { entries: Vec::new() }
        }
        
        fn add(&mut self, value: i32) {
            self.entries.push(Bucket { hash: 0, key: value, value: () });
        }
    }

    impl IndexSet {
        pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<i32>
        where
            F: FnMut(&i32, &i32) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let mut set = IndexSet::new();
    set.add(3);
    set.add(1);
    set.add(2);

    let sorted_iter = set.sorted_by(|a, b| a.cmp(b));

    let results: Vec<i32> = sorted_iter.iter.collect();
    assert_eq!(results, vec![1, 2, 3]);
} 

#[test]
fn test_sorted_by_stability() {
    struct IndexSet {
        entries: Vec<Bucket<(i32, char), ()>>,
    }
    
    impl IndexSet {
        fn into_entries(self) -> Vec<Bucket<(i32, char), ()>> {
            self.entries
        }
        
        fn new() -> Self {
            IndexSet { entries: Vec::new() }
        }
        
        fn add(&mut self, value: (i32, char)) {
            self.entries.push(Bucket { hash: 0, key: value, value: () });
        }
    }

    impl IndexSet {
        pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<(i32, char)>
        where
            F: FnMut(&(i32, char), &(i32, char)) -> Ordering,
        {
            let mut entries = self.into_entries();
            entries.sort_by(move |a, b| cmp(&a.key, &b.key));
            IntoIter::new(entries)
        }
    }

    let mut set = IndexSet::new();
    set.add((1, 'a'));
    set.add((2, 'b'));
    set.add((1, 'c'));

    let sorted_iter = set.sorted_by(|a, b| a.0.cmp(&b.0));

    let results: Vec<(i32, char)> = sorted_iter.iter.collect();
    assert_eq!(results, vec![(1, 'a'), (1, 'c'), (2, 'b')]);
}

