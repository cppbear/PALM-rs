// Answer 0

#[test]
fn test_contains_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + std::cmp::PartialEq<i32>,
        {
            self.map.contains(value)
        }
    }

    let set = MySet { map: HashSet::new() };
    set.map.insert(1);
    set.map.insert(2);
    assert!(set.contains(&1));
}

#[test]
fn test_contains_non_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + std::cmp::PartialEq<i32>,
        {
            self.map.contains(value)
        }
    }

    let set = MySet { map: HashSet::new() };
    set.map.insert(1);
    set.map.insert(2);
    assert!(!set.contains(&3));
}

#[test]
fn test_contains_boundary_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        pub fn contains<Q>(&self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + std::cmp::PartialEq<i32>,
        {
            self.map.contains(value)
        }
    }

    let mut set = MySet { map: HashSet::new() };
    set.map.insert(0);
    set.map.insert(100);
    assert!(set.contains(&0));
    assert!(set.contains(&100));
    assert!(!set.contains(&50));
}

