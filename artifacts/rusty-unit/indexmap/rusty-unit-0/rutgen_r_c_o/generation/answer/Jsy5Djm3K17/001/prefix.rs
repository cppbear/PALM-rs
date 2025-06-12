// Answer 0

#[test]
fn test_symmetric_difference_debug_format() {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;

    struct TestElement {
        value: usize,
    }

    impl Eq for TestElement {}
    impl Hash for TestElement {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let set1: IndexSet<TestElement, RandomState> = IndexSet::from_iter((1..=500).map(|x| TestElement { value: x }));
    let set2: IndexSet<TestElement, RandomState> = IndexSet::from_iter((250..=750).map(|x| TestElement { value: x }));

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let _ = fmt::Debug::fmt(&symmetric_difference, &mut fmt::Formatter::default());
}

#[test]
fn test_empty_sets_debug_format() {
    use std::collections::hash_map::RandomState;

    struct TestElement {
        value: usize,
    }

    impl Eq for TestElement {}
    impl Hash for TestElement {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let set1: IndexSet<TestElement, RandomState> = IndexSet::new();
    let set2: IndexSet<TestElement, RandomState> = IndexSet::new();

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let _ = fmt::Debug::fmt(&symmetric_difference, &mut fmt::Formatter::default());
}

#[test]
fn test_one_element_sets_debug_format() {
    use std::collections::hash_map::RandomState;

    struct TestElement {
        value: usize,
    }

    impl Eq for TestElement {}
    impl Hash for TestElement {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let set1: IndexSet<TestElement, RandomState> = IndexSet::from_iter(std::iter::once(TestElement { value: 1 }));
    let set2: IndexSet<TestElement, RandomState> = IndexSet::from_iter(std::iter::once(TestElement { value: 2 }));

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let _ = fmt::Debug::fmt(&symmetric_difference, &mut fmt::Formatter::default());
}

#[test]
fn test_largest_sets_debug_format() {
    use std::collections::hash_map::RandomState;

    struct TestElement {
        value: usize,
    }

    impl Eq for TestElement {}
    impl Hash for TestElement {
        fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }

    let set1: IndexSet<TestElement, RandomState> = IndexSet::from_iter((1..=1000).map(|x| TestElement { value: x }));
    let set2: IndexSet<TestElement, RandomState> = IndexSet::from_iter((501..=1500).map(|x| TestElement { value: x }));

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    let _ = fmt::Debug::fmt(&symmetric_difference, &mut fmt::Formatter::default());
}

