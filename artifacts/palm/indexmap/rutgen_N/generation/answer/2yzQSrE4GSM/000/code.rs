// Answer 0

#[test]
fn test_swap_take_exists() {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn swap_take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + Hash + PartialEq,
        {
            if self.map.contains(value) {
                let mut val = value.clone();
                self.map.remove(&val);
                return Some(val);
            }
            None
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);
    set.map.insert(3);

    assert_eq!(set.swap_take(&2), Some(2));
    assert_eq!(set.map.len(), 2);
}

#[test]
fn test_swap_take_not_exists() {
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

        pub fn swap_take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + Hash + PartialEq,
        {
            if self.map.contains(value) {
                let mut val = value.clone();
                self.map.remove(&val);
                return Some(val);
            }
            None
        }
    }

    let mut set = MySet::new();
    set.map.insert(1);
    set.map.insert(2);
    
    assert_eq!(set.swap_take(&3), None);
    assert_eq!(set.map.len(), 2);
}

