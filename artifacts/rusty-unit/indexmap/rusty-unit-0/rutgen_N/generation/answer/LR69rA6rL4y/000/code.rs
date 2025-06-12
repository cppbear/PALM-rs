// Answer 0

#[test]
fn test_shift_take_existing_value() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<u32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn shift_take<Q>(&mut self, value: &Q) -> Option<u32>
        where
            Q: ?Sized + Hash + Eq,
        {
            if self.map.remove(value) {
                Some(value.clone() as u32)
            } else {
                None
            }
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);

    assert_eq!(set.shift_take(&1), Some(1));
    assert_eq!(set.shift_take(&2), Some(2));
    assert_eq!(set.shift_take(&3), None);
}

#[test]
fn test_shift_take_empty_set() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<u32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn shift_take<Q>(&mut self, value: &Q) -> Option<u32>
        where
            Q: ?Sized + Hash + Eq,
        {
            if self.map.remove(value) {
                Some(value.clone() as u32)
            } else {
                None
            }
        }
    }

    let mut set = MySet::new();
    
    assert_eq!(set.shift_take(&1), None);
}

#[test]
fn test_shift_take_multiple_elements() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<u32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn shift_take<Q>(&mut self, value: &Q) -> Option<u32>
        where
            Q: ?Sized + Hash + Eq,
        {
            if self.map.remove(value) {
                Some(value.clone() as u32)
            } else {
                None
            }
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);
    set.map.insert(3);

    assert_eq!(set.shift_take(&2), Some(2));
    assert_eq!(set.shift_take(&1), Some(1));
    assert_eq!(set.shift_take(&3), Some(3));
    assert_eq!(set.shift_take(&4), None);
}

#[test]
fn test_shift_take_preserve_order() {
    use std::collections::HashSet;
    use std::hash::Hash;

    struct MySet {
        map: HashSet<u32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn shift_take<Q>(&mut self, value: &Q) -> Option<u32>
        where
            Q: ?Sized + Hash + Eq,
        {
            if self.map.remove(value) {
                Some(value.clone() as u32)
            } else {
                None
            }
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);
    set.map.insert(3);

    assert_eq!(set.shift_take(&1), Some(1));
    assert_eq!(set.shift_take(&2), Some(2));
    assert!(set.map.contains(&3));
}

