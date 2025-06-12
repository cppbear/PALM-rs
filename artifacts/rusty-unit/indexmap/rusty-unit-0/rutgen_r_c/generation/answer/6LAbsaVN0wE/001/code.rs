// Answer 0

#[test]
fn test_symmetric_difference_with_disjoint_sets() {
    use std::collections::hash_map::RandomState;
    
    struct MyIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }
    
    impl MyIndexSet {
        fn new() -> Self {
            Self {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn symmetric_difference(&self, other: &Self) -> SymmetricDifference<i32, RandomState, RandomState> {
            self.symmetric_difference(other)
        }
    }

    let mut set1 = MyIndexSet::new();
    let mut set2 = MyIndexSet::new();

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    set2.insert(4);
    set2.insert(5);
    
    let result = set1.symmetric_difference(&set2);
    
    assert_eq!(result.collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_symmetric_difference_with_empty_sets() {
    use std::collections::hash_map::RandomState;
    
    struct MyIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }

    impl MyIndexSet {
        fn new() -> Self {
            Self {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn symmetric_difference(&self, other: &Self) -> SymmetricDifference<i32, RandomState, RandomState> {
            self.symmetric_difference(other)
        }
    }

    let set1 = MyIndexSet::new();
    let set2 = MyIndexSet::new();

    let result = set1.symmetric_difference(&set2);
    
    assert!(result.collect::<Vec<_>>().is_empty());
}

#[test]
fn test_symmetric_difference_with_identical_sets() {
    use std::collections::hash_map::RandomState;
    
    struct MyIndexSet {
        map: IndexMap<i32, (), RandomState>,
    }

    impl MyIndexSet {
        fn new() -> Self {
            Self {
                map: IndexMap::new(),
            }
        }

        fn insert(&mut self, value: i32) {
            self.map.insert(value, ());
        }

        fn symmetric_difference(&self, other: &Self) -> SymmetricDifference<i32, RandomState, RandomState> {
            self.symmetric_difference(other)
        }
    }

    let mut set1 = MyIndexSet::new();
    let mut set2 = MyIndexSet::new();

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    
    let result = set1.symmetric_difference(&set2);
    
    assert!(result.collect::<Vec<_>>().is_empty());
}

