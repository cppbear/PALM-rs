// Answer 0

#[test]
fn test_shift_remove_full_existing_value() {
    struct MySet {
        map: std::collections::HashMap<i32, ()>,
    }
    
    impl MySet {
        fn new() -> Self {
            MySet { map: std::collections::HashMap::new() }
        }

        fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, i32)>
        where
            Q: ?Sized + std::hash::Hash + Equivalent<i32>,
        {
            let index = self.map.keys().position(|&k| k == *value)?;
            self.map.remove(value);
            Some((index, *value))
        }
    }

    let mut set = MySet::new();
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    assert_eq!(set.shift_remove_full(&2), Some((1, 2)));
    assert!(!set.map.contains_key(&2));
}

#[test]
fn test_shift_remove_full_non_existing_value() {
    struct MySet {
        map: std::collections::HashMap<i32, ()>,
    }
    
    impl MySet {
        fn new() -> Self {
            MySet { map: std::collections::HashMap::new() }
        }

        fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, i32)>
        where
            Q: ?Sized + std::hash::Hash + Equivalent<i32>,
        {
            let index = self.map.keys().position(|&k| k == *value)?;
            self.map.remove(value);
            Some((index, *value))
        }
    }

    let mut set = MySet::new();
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    assert_eq!(set.shift_remove_full(&4), None);
}

#[test]
fn test_shift_remove_full_multiple_removals() {
    struct MySet {
        map: std::collections::HashMap<i32, ()>,
    }
    
    impl MySet {
        fn new() -> Self {
            MySet { map: std::collections::HashMap::new() }
        }

        fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, i32)>
        where
            Q: ?Sized + std::hash::Hash + Equivalent<i32>,
        {
            let index = self.map.keys().position(|&k| k == *value)?;
            self.map.remove(value);
            Some((index, *value))
        }
    }

    let mut set = MySet::new();
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());

    assert_eq!(set.shift_remove_full(&1), Some((0, 1)));
    assert_eq!(set.shift_remove_full(&3), Some((1, 3))); // The index of 3 changes after removing 1
}

