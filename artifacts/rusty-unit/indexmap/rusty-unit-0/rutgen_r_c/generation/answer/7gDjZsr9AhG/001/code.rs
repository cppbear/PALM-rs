// Answer 0

#[test]
fn test_swap_remove_full_existing_value() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            for (i, v) in self.values.iter_mut().enumerate() {
                if *v == *value {
                    return Some((i, v));
                }
            }
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            self.values.retain(keep);
        }
    }

    let mut set = TestSet { values: vec![1, 2, 3] };
    let (index, value) = set.swap_remove_full(&2).unwrap();
    assert_eq!(index, 1);
    assert_eq!(value, 2);
    assert_eq!(set.values, vec![1, 3]);
}

#[test]
fn test_swap_remove_full_non_existing_value() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            for (i, v) in self.values.iter_mut().enumerate() {
                if *v == *value {
                    return Some((i, v));
                }
            }
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            self.values.retain(keep);
        }
    }

    let mut set = TestSet { values: vec![1, 2, 3] };
    let result = set.swap_remove_full(&4);
    assert!(result.is_none());
}

#[test]
fn test_swap_remove_full_empty_set() {
    struct TestSet {
        values: Vec<i32>,
    }

    impl MutableValues for TestSet {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            for (i, v) in self.values.iter_mut().enumerate() {
                if *v == *value {
                    return Some((i, v));
                }
            }
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            self.values.retain(keep);
        }
    }

    let mut set = TestSet { values: vec![] };
    let result = set.swap_remove_full(&1);
    assert!(result.is_none());
}

