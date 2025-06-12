// Answer 0

#[test]
fn test_swap_take_existing_element() {
    struct MySet {
        elements: IndexSet<i32, RandomState>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                elements: IndexSet {
                    map: IndexMap {
                        core: IndexMapCore::new(),
                        hash_builder: RandomState::new(),
                    },
                },
            }
        }

        fn insert(&mut self, value: i32) {
            self.elements.map.insert(value, ());
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert_eq!(set.elements.swap_take(&1), Some(1));
    assert_eq!(set.elements.swap_take(&2), Some(2));
    assert_eq!(set.elements.swap_take(&3), Some(3));
}

#[test]
fn test_swap_take_non_existing_element() {
    struct MySet {
        elements: IndexSet<i32, RandomState>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                elements: IndexSet {
                    map: IndexMap {
                        core: IndexMapCore::new(),
                        hash_builder: RandomState::new(),
                    },
                },
            }
        }

        fn insert(&mut self, value: i32) {
            self.elements.map.insert(value, ());
        }
    }

    let mut set = MySet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    assert_eq!(set.elements.swap_take(&40), None);
}

#[test]
fn test_swap_take_multiple_identical_elements() {
    struct MySet {
        elements: IndexSet<i32, RandomState>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                elements: IndexSet {
                    map: IndexMap {
                        core: IndexMapCore::new(),
                        hash_builder: RandomState::new(),
                    },
                },
            }
        }

        fn insert(&mut self, value: i32) {
            self.elements.map.insert(value, ());
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(1);
    set.insert(1);

    assert_eq!(set.elements.swap_take(&1), Some(1));
    assert_eq!(set.elements.swap_take(&1), Some(1)); // Removal still leaves one instance
    assert_eq!(set.elements.swap_take(&1), Some(1)); // Last instance removed
    assert_eq!(set.elements.swap_take(&1), None); // No more instances left
}

#[should_panic]
#[test]
fn test_swap_take_removing_from_empty_set() {
    struct MySet {
        elements: IndexSet<i32, RandomState>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                elements: IndexSet {
                    map: IndexMap {
                        core: IndexMapCore::new(),
                        hash_builder: RandomState::new(),
                    },
                },
            }
        }
    }

    let mut set = MySet::new();
    set.elements.swap_take(&100); // Attempt to remove from empty set, should panic if not handled properly
}

