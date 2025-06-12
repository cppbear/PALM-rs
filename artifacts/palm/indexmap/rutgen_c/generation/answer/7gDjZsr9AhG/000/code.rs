// Answer 0

#[test]
fn test_swap_remove_full_existing_value() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 { 0 }
    }
    
    struct TestIndexMap {
        values: Vec<i32>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap { values: Vec::new() }
        }
        
        fn swap_remove_full<Q>(&mut self, _value: &Q) -> Option<(usize, i32, ())>
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            if !self.values.is_empty() {
                let last = self.values.len() - 1;
                let value = self.values.swap_remove(last);
                Some((last, value, ()))
            } else {
                None
            }
        }
    }

    let mut set = IndexSet {
        map: TestIndexMap::new(),
    };
    set.map.values.push(1);
    set.map.values.push(2);
    
    let result = set.swap_remove_full(&1);
    assert_eq!(result, Some((0, 1)));
    assert_eq!(set.map.values.len(), 1);
    assert_eq!(set.map.values[0], 2);
}

#[test]
fn test_swap_remove_full_non_existing_value() {
    struct TestHasher;
    use std::hash::{Hash, Hasher};

    impl Hasher for TestHasher {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 { 0 }
    }
    
    struct TestIndexMap {
        values: Vec<i32>,
    }
    
    impl TestIndexMap {
        fn new() -> Self {
            TestIndexMap { values: Vec::new() }
        }
        
        fn swap_remove_full<Q>(&mut self, _value: &Q) -> Option<(usize, i32, ())>
        where
            Q: ?Sized + Hash + Equivalent<i32>,
        {
            None // Simulate non-existing value case
        }
    }

    let mut set = IndexSet {
        map: TestIndexMap::new(),
    };
    set.map.values.push(2);

    let result = set.swap_remove_full(&1);
    assert_eq!(result, None);
    assert_eq!(set.map.values.len(), 1);
}

