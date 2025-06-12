// Answer 0

#[test]
fn test_swap_remove_existing_value() {
    use std::collections::HashSet;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + std::hash::Hash + std::cmp::Eq,
        {
            self.map.remove(value)
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);
    set.map.insert(3);

    assert!(set.swap_remove(&2));
    assert!(!set.map.contains(&2));
}

#[test]
fn test_swap_remove_non_existing_value() {
    use std::collections::HashSet;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + std::hash::Hash + std::cmp::Eq,
        {
            self.map.remove(value)
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);
    set.map.insert(3);

    assert!(!set.swap_remove(&4));
}

#[test]
fn test_swap_remove_empty_set() {
    use std::collections::HashSet;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + std::hash::Hash + std::cmp::Eq,
        {
            self.map.remove(value)
        }
    }

    let mut set = MySet::new();
    assert!(!set.swap_remove(&1));
}

