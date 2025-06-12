// Answer 0

#[test]
fn test_is_subset_not_subset() {
    use std::collections::hash_map::RandomState;
    
    struct MySet {
        elements: Vec<i32>,
        hasher: RandomState,
    }
    
    impl MySet {
        fn new(elements: Vec<i32>) -> Self {
            MySet {
                elements,
                hasher: RandomState::new(),
            }
        }
        
        fn as_index_set(&self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_hasher(self.hasher.clone());
            for &elem in &self.elements {
                set.insert(elem);
            }
            set
        }
    }

    let set_a = MySet::new(vec![1, 2, 3]).as_index_set();
    let set_b = MySet::new(vec![1, 2]).as_index_set(); // set_b is smaller than set_a

    assert!(!set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_empty_subset() {
    use std::collections::hash_map::RandomState;

    struct MySet {
        elements: Vec<i32>,
        hasher: RandomState,
    }
    
    impl MySet {
        fn new(elements: Vec<i32>) -> Self {
            MySet {
                elements,
                hasher: RandomState::new(),
            }
        }
        
        fn as_index_set(&self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_hasher(self.hasher.clone());
            for &elem in &self.elements {
                set.insert(elem);
            }
            set
        }
    }

    let set_a = MySet::new(vec![1, 2, 3]).as_index_set();
    let set_b = MySet::new(vec![]).as_index_set(); // set_b is empty

    assert!(!set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_equal_sets() {
    use std::collections::hash_map::RandomState;

    struct MySet {
        elements: Vec<i32>,
        hasher: RandomState,
    }
    
    impl MySet {
        fn new(elements: Vec<i32>) -> Self {
            MySet {
                elements,
                hasher: RandomState::new(),
            }
        }
        
        fn as_index_set(&self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_hasher(self.hasher.clone());
            for &elem in &self.elements {
                set.insert(elem);
            }
            set
        }
    }

    let set_a = MySet::new(vec![1, 2, 3]).as_index_set();
    let set_b = MySet::new(vec![1, 2, 3]).as_index_set(); // set_b is equal to set_a

    assert!(set_a.is_subset(&set_b));
} 

#[test]
fn test_is_subset_partial_subset() {
    use std::collections::hash_map::RandomState;

    struct MySet {
        elements: Vec<i32>,
        hasher: RandomState,
    }
    
    impl MySet {
        fn new(elements: Vec<i32>) -> Self {
            MySet {
                elements,
                hasher: RandomState::new(),
            }
        }
        
        fn as_index_set(&self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_hasher(self.hasher.clone());
            for &elem in &self.elements {
                set.insert(elem);
            }
            set
        }
    }

    let set_a = MySet::new(vec![1, 2]).as_index_set();
    let set_b = MySet::new(vec![1, 2, 3]).as_index_set(); // set_b contains all elements of set_a

    assert!(set_a.is_subset(&set_b));
}

