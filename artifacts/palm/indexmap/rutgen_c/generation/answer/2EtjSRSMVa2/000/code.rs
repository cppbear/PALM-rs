// Answer 0

#[test]
fn test_shift_remove_finish_removes_entry_and_decrements_indices() {
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut indices = vec![1, 2, 3];
    
    let initial_entries = vec![
        Bucket { hash: HashValue::default(), key: 0, value: 10 },
        Bucket { hash: HashValue::default(), key: 1, value: 20 },
        Bucket { hash: HashValue::default(), key: 2, value: 30 },
    ];

    let mut test_entries = TestEntries {
        entries: initial_entries,
    };

    let mut ref_mut = RefMut::new(&mut indices, &mut test_entries);

    let removed_entry = ref_mut.shift_remove_finish(1);
    
    assert_eq!(removed_entry, (1, 20));
    assert_eq!(test_entries.entries.len(), 2);
    assert_eq!(test_entries.entries[0], Bucket { hash: HashValue::default(), key: 0, value: 10 });
    assert_eq!(test_entries.entries[1], Bucket { hash: HashValue::default(), key: 2, value: 30 });
    assert_eq!(indices, vec![1, 2]); // Ensure indices are correctly decremented.
} 

#[test]
fn test_shift_remove_finish_boundary_conditions() {
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
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries);
        }
    }

    let mut indices = vec![0, 1];

    let initial_entries = vec![
        Bucket { hash: HashValue::default(), key: 0, value: 10 },
        Bucket { hash: HashValue::default(), key: 1, value: 20 },
    ];

    let mut test_entries = TestEntries {
        entries: initial_entries,
    };

    let mut ref_mut = RefMut::new(&mut indices, &mut test_entries);

    let removed_entry = ref_mut.shift_remove_finish(0);

    assert_eq!(removed_entry, (0, 10));
    assert_eq!(test_entries.entries.len(), 1);
    assert_eq!(test_entries.entries[0], Bucket { hash: HashValue::default(), key: 1, value: 20 });
    assert_eq!(indices, vec![0]); // Ensure indices are correctly updated after removal.
}

