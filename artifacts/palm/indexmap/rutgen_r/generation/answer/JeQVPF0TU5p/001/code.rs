// Answer 0

#[test]
fn test_get_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MockSet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MockSet<T> {
        pub fn new() -> Self {
            MockSet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.map.insert(value);
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Hash + std::cmp::PartialEq + Equivalent<T>,
        {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    let mut set = MockSet::new();
    set.insert(1);
    assert_eq!(set.get(&1), Some(&1));
}

#[test]
fn test_get_non_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MockSet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MockSet<T> {
        pub fn new() -> Self {
            MockSet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.map.insert(value);
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Hash + std::cmp::PartialEq + Equivalent<T>,
        {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    let set = MockSet::new();
    assert_eq!(set.get(&1), None);
}

#[test]
fn test_get_nil_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MockSet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MockSet<T> {
        pub fn new() -> Self {
            MockSet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.map.insert(value);
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Hash + std::cmp::PartialEq + Equivalent<T>,
        {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    let mut set = MockSet::new();
    set.insert(0);
    assert_eq!(set.get(&0), Some(&0));
    assert_eq!(set.get(&-1), None);
}

