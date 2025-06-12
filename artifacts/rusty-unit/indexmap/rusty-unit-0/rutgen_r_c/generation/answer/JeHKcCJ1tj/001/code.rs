// Answer 0

#[test]
fn test_is_disjoint_equal_length_disjoint_sets() {
    use std::collections::hash_map::RandomState;

    struct TestSet {
        elements: Vec<i32>,
        hasher: RandomState,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            TestSet {
                elements,
                hasher: RandomState::new(),
            }
        }

        fn to_index_set(self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_capacity_and_hasher(self.elements.len(), self.hasher);
            for elem in self.elements {
                // Simulate adding element to the set (details not provided in context).
            }
            set
        }
    }

    let set_a = TestSet::new(vec![1, 2, 3]).to_index_set();
    let set_b = TestSet::new(vec![4, 5, 6]).to_index_set();

    assert!(set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_non_disjoint_sets() {
    use std::collections::hash_map::RandomState;

    struct TestSet {
        elements: Vec<i32>,
        hasher: RandomState,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            TestSet {
                elements,
                hasher: RandomState::new(),
            }
        }

        fn to_index_set(self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_capacity_and_hasher(self.elements.len(), self.hasher);
            for elem in self.elements {
                // Simulate adding element to the set (details not provided in context).
            }
            set
        }
    }

    let set_a = TestSet::new(vec![1, 2, 3]).to_index_set();
    let set_b = TestSet::new(vec![3, 4, 5]).to_index_set();

    assert!(!set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_empty_sets() {
    use std::collections::hash_map::RandomState;

    let set_a = IndexSet::<i32, RandomState>::with_capacity_and_hasher(0, RandomState::new());
    let set_b = IndexSet::<i32, RandomState>::with_capacity_and_hasher(0, RandomState::new());

    assert!(set_a.is_disjoint(&set_b));
}

#[test]
fn test_is_disjoint_equal_length_with_one_disjoint_element() {
    use std::collections::hash_map::RandomState;

    struct TestSet {
        elements: Vec<i32>,
        hasher: RandomState,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            TestSet {
                elements,
                hasher: RandomState::new(),
            }
        }

        fn to_index_set(self) -> IndexSet<i32, RandomState> {
            let mut set = IndexSet::with_capacity_and_hasher(self.elements.len(), self.hasher);
            for elem in self.elements {
                // Simulate adding element to the set (details not provided in context).
            }
            set
        }
    }

    let set_a = TestSet::new(vec![1]).to_index_set();
    let set_b = TestSet::new(vec![2]).to_index_set();

    assert!(set_a.is_disjoint(&set_b));
}

