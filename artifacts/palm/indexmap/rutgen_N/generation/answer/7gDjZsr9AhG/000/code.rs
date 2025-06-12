// Answer 0

#[test]
fn test_swap_remove_full_existing_value() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    struct TestValue(usize);

    impl Hash for TestValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for TestValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for TestValue {}

    let mut set = IndexSet::new();
    set.insert(TestValue(1));
    set.insert(TestValue(2));
    set.insert(TestValue(3));

    let result = set.swap_remove_full(&TestValue(2));
    assert_eq!(result, Some((1, TestValue(2))));
    assert!(!set.contains(&TestValue(2)));
}

#[test]
fn test_swap_remove_full_non_existing_value() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    struct TestValue(usize);

    impl Hash for TestValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for TestValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for TestValue {}

    let mut set = IndexSet::new();
    set.insert(TestValue(1));
    set.insert(TestValue(2));

    let result = set.swap_remove_full(&TestValue(3));
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_full_boundary_conditions() {
    use indexmap::IndexSet;
    use std::hash::Hash;

    struct TestValue(usize);

    impl Hash for TestValue {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for TestValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for TestValue {}

    let mut set = IndexSet::new();

    let result = set.swap_remove_full(&TestValue(1));
    assert_eq!(result, None);

    set.insert(TestValue(1));
    let result = set.swap_remove_full(&TestValue(1));
    assert_eq!(result, Some((0, TestValue(1))));
    assert!(set.is_empty());
}

