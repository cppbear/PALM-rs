// Answer 0

#[test]
fn test_swap_remove_existing_element() {
    struct TestSet {
        inner: super::IndexSet<i32, std::collections::hash_map::RandomState>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                inner: super::IndexSet {
                    map: super::IndexMap {
                        core: super::IndexMapCore::new(),
                        hash_builder: std::collections::hash_map::RandomState::new(),
                    },
                },
            }
        }
    }

    let mut set = TestSet::new();
    set.inner.insert(1);
    set.inner.insert(2);

    assert!(set.inner.swap_remove(&1));
    assert!(!set.inner.contains(&1));
    assert!(set.inner.contains(&2));
}

#[test]
fn test_swap_remove_non_existing_element() {
    struct TestSet {
        inner: super::IndexSet<i32, std::collections::hash_map::RandomState>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                inner: super::IndexSet {
                    map: super::IndexMap {
                        core: super::IndexMapCore::new(),
                        hash_builder: std::collections::hash_map::RandomState::new(),
                    },
                },
            }
        }
    }

    let mut set = TestSet::new();
    set.inner.insert(1);
    set.inner.insert(2);

    assert!(!set.inner.swap_remove(&3));
}

#[test]
fn test_swap_remove_last_element() {
    struct TestSet {
        inner: super::IndexSet<i32, std::collections::hash_map::RandomState>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                inner: super::IndexSet {
                    map: super::IndexMap {
                        core: super::IndexMapCore::new(),
                        hash_builder: std::collections::hash_map::RandomState::new(),
                    },
                },
            }
        }
    }

    let mut set = TestSet::new();
    set.inner.insert(1);
    set.inner.insert(2);

    assert!(set.inner.swap_remove(&2)); // Remove the last element
    assert!(!set.inner.contains(&2));
    assert!(set.inner.contains(&1));
}

