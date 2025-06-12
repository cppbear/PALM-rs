// Answer 0

#[test]
fn test_is_disjoint_when_self_len_greater_than_other_len() {
    use std::collections::hash_map::RandomState;

    struct TestSet {
        set: super::IndexSet<i32, RandomState>,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            let mut set = super::IndexSet::with_capacity_and_hasher(elements.len(), RandomState::new());
            for element in elements {
                set.insert(element);
            }
            Self { set }
        }

        fn insert(&mut self, value: i32) {
            self.set.insert(value);
        }

        fn len(&self) -> usize {
            self.set.len()
        }

        fn contains(&self, value: &i32) -> bool {
            self.set.contains(value)
        }

        fn clear(&mut self) {
            self.set.clear();
        }
    }

    // Create two IndexSets, with the first having more elements than the second
    let mut set_a = TestSet::new(vec![1, 2, 3]);
    let mut set_b = TestSet::new(vec![4, 5]);

    // Check that set_a is not disjoint with set_b (should return false)
    assert_eq!(set_a.is_disjoint(&set_b.set), true);

    // Introduce an element into set_b that is also in set_a
    set_b.insert(2);

    // Now check that set_a is disjoint with set_b (should return false)
    assert_eq!(set_a.is_disjoint(&set_b.set), false);
}

#[test]
fn test_is_disjoint_when_self_len_is_equal_to_other_len() {
    use std::collections::hash_map::RandomState;

    struct TestSet {
        set: super::IndexSet<i32, RandomState>,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            let mut set = super::IndexSet::with_capacity_and_hasher(elements.len(), RandomState::new());
            for element in elements {
                set.insert(element);
            }
            Self { set }
        }

        fn contains(&self, value: &i32) -> bool {
            self.set.contains(value)
        }
    }

    let set_a = TestSet::new(vec![1, 2, 3]);
    let set_b = TestSet::new(vec![4, 5, 6]);

    // Check that set_a is disjoint with set_b (should return true)
    assert_eq!(set_a.is_disjoint(&set_b.set), true);
}

#[test]
fn test_is_disjoint_with_common_elements() {
    use std::collections::hash_map::RandomState;

    struct TestSet {
        set: super::IndexSet<i32, RandomState>,
    }

    impl TestSet {
        fn new(elements: Vec<i32>) -> Self {
            let mut set = super::IndexSet::with_capacity_and_hasher(elements.len(), RandomState::new());
            for element in elements {
                set.insert(element);
            }
            Self { set }
        }
    }

    let set_a = TestSet::new(vec![1, 2, 3]);
    let mut set_b = TestSet::new(vec![2, 4, 5]);

    // Check that set_a is not disjoint with set_b (should return false)
    assert_eq!(set_a.is_disjoint(&set_b.set), false);
}

