// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hash;
    use std::collections::HashSet;

    #[derive(Debug)]
    struct MySet<T> {
        map: HashSet<T>,
    }

    impl<T: Eq + Hash> MySet<T> {
        pub fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn shift_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<T>,
        {
            self.map.remove(value)
        }
    }

    #[test]
    fn test_shift_remove_present_value() {
        let mut my_set = MySet::new();
        my_set.map.insert(1);
        my_set.map.insert(2);
        my_set.map.insert(3);
        
        assert!(my_set.shift_remove(&2));
        assert!(!my_set.map.contains(&2));
    }

    #[test]
    fn test_shift_remove_absent_value() {
        let mut my_set = MySet::new();
        my_set.map.insert(1);
        my_set.map.insert(3);

        assert!(!my_set.shift_remove(&2));
        assert!(my_set.map.contains(&1));
        assert!(my_set.map.contains(&3));
    }

    #[test]
    fn test_shift_remove_multiple_and_order() {
        let mut my_set = MySet::new();
        my_set.map.insert(1);
        my_set.map.insert(2);
        my_set.map.insert(3);
        
        assert!(my_set.shift_remove(&1));
        assert!(my_set.shift_remove(&3));
        assert!(!my_set.map.contains(&1));
        assert!(!my_set.map.contains(&3));
        assert!(my_set.map.contains(&2));
    }

    #[test]
    fn test_shift_remove_on_empty_set() {
        let mut my_set = MySet::new();
        
        assert!(!my_set.shift_remove(&1));
    }
}

