// Answer 0

#[test]
fn test_try_reserve_exact_success() {
    struct TestEntries;
    
    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            vec![]
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }
        
        fn with_entries<F>(&mut self, _f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut map = IndexMapCore::<usize, usize>::new();
    let result = map.try_reserve_exact(10);
    assert!(result.is_ok());
}

#[test]
fn test_try_reserve_exact_over_capacity() {
    struct TestEntries;
    
    impl Entries for TestEntries {
        type Entry = Bucket<usize, usize>;
        
        fn into_entries(self) -> Vec<Self::Entry> {
            vec![]
        }
        
        fn as_entries(&self) -> &[Self::Entry] {
            &[]
        }
        
        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut []
        }
        
        fn with_entries<F>(&mut self, _f: F)
        where
            F: FnOnce(&mut [Self::Entry]) {}
    }

    let mut map = IndexMapCore::<usize, usize>::with_capacity(5);
    let _ = map.try_reserve_exact(5); // Pre-fill to current capacity.
    let result = map.try_reserve_exact(10); // Exceeding the capacity.
    assert!(result.is_err());
}

