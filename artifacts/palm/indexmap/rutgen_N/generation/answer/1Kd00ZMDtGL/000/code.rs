// Answer 0

#[test]
fn test_take_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        set: HashSet<i32>,
    }

    impl MySet {
        pub fn new() -> Self {
            MySet {
                set: HashSet::new(),
            }
        }

        pub fn take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + Hash + std::cmp::Eq,
        {
            if self.set.contains(value) {
                self.set.take(value)
            } else {
                None
            }
        }
    }

    let mut my_set = MySet::new();
    my_set.set.insert(1);
    my_set.set.insert(2);
    assert_eq!(my_set.take(&1), Some(1));
    assert_eq!(my_set.take(&1), None);
}

#[test]
fn test_take_non_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        set: HashSet<i32>,
    }

    impl MySet {
        pub fn new() -> Self {
            MySet {
                set: HashSet::new(),
            }
        }

        pub fn take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + Hash + std::cmp::Eq,
        {
            if self.set.contains(value) {
                self.set.take(value)
            } else {
                None
            }
        }
    }

    let mut my_set = MySet::new();
    my_set.set.insert(1);
    my_set.set.insert(2);
    assert_eq!(my_set.take(&3), None);
}

#[test]
fn test_take_from_empty_set() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        set: HashSet<i32>,
    }

    impl MySet {
        pub fn new() -> Self {
            MySet {
                set: HashSet::new(),
            }
        }

        pub fn take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + Hash + std::cmp::Eq,
        {
            if self.set.contains(value) {
                self.set.take(value)
            } else {
                None
            }
        }
    }

    let mut my_set = MySet::new();
    assert_eq!(my_set.take(&1), None);
}

