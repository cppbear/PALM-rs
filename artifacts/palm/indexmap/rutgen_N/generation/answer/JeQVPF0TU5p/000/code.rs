// Answer 0

#[test]
fn test_get_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MySet<T> {
        pub fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.map.insert(value);
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Hash + Equivalent<T>,
        {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    impl<T> Equivalent<T> for T where T: Eq {
        fn equivalent(&self, other: &T) -> bool {
            self == other
        }
    }

    let mut my_set = MySet::new();
    my_set.insert(42);

    assert_eq!(my_set.get(&42), Some(&42));
}

#[test]
fn test_get_non_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MySet<T> {
        pub fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.map.insert(value);
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Hash + Equivalent<T>,
        {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    impl<T> Equivalent<T> for T where T: Eq {
        fn equivalent(&self, other: &T) -> bool {
            self == other
        }
    }

    let my_set = MySet::<i32>::new();

    assert_eq!(my_set.get(&100), None);
}

#[test]
fn test_get_with_different_type() {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    struct MySet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MySet<T> {
        pub fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: T) {
            self.map.insert(value);
        }

        pub fn get<Q>(&self, value: &Q) -> Option<&T>
        where
            Q: ?Sized + Hash + Equivalent<T>,
        {
            self.map.get_key_value(value).map(|(x, &())| x)
        }
    }

    impl<T> Equivalent<T> for T where T: Eq {
        fn equivalent(&self, other: &T) -> bool {
            self == other
        }
    }

    let mut my_set = MySet::new();
    my_set.insert(7);

    assert_eq!(my_set.get(&7), Some(&7));
    assert_eq!(my_set.get(&"not an int"), None);
}

