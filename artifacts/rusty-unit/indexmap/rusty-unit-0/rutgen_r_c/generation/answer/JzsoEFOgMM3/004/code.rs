// Answer 0

#[test]
fn test_decrement_indices_large_shift() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }
        
        fn with_entries<F>(&mut self, f: F) 
        where 
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut indices = vec![3, 4, 5, 6].into_iter().collect::<Indices>();
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: 10 },
            Bucket { hash: HashValue(2), key: 2, value: 20 },
            Bucket { hash: HashValue(3), key: 3, value: 30 },
            Bucket { hash: HashValue(4), key: 4, value: 40 },
            Bucket { hash: HashValue(5), key: 5, value: 50 },
        ],
    };
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.decrement_indices(1, 4); // shift indices from 1 to 3 down

    assert_eq!(indices, vec![3, 4, 4, 5].into_iter().collect::<Indices>());
}

#[test]
#[should_panic(expected = "index not found")]
fn test_decrement_indices_panic() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }
        
        fn with_entries<F>(&mut self, f: F) 
        where 
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut indices = vec![0, 1, 2].into_iter().collect::<Indices>();
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: 10 },
            Bucket { hash: HashValue(2), key: 2, value: 20 },
        ],
    };
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // This will panic due to trying to access an out-of-bounds entry
    ref_mut.decrement_indices(1, 3);
}

#[test]
fn test_decrement_indices_small_shift() {
    struct TestEntries {
        entries: Vec<Bucket<usize, usize>>,
    }
    
    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }
        
        fn with_entries<F>(&mut self, f: F) 
        where 
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut indices = vec![0, 1, 2].into_iter().collect::<Indices>();
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: 1, value: 10 },
            Bucket { hash: HashValue(2), key: 2, value: 20 },
        ],
    };
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.decrement_indices(0, 2); // this will not cause panic

    assert_eq!(indices, vec![0, 1, 1].into_iter().collect::<Indices>());
}

