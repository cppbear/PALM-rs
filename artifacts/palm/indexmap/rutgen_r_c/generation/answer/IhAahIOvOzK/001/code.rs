// Answer 0

#[test]
fn test_last_on_non_empty_index_set() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            // No-op
        }
    }

    let mut index_set = TestMutableValues { values: vec![1, 2, 3] };
    assert_eq!(index_set.last(), Some(&3));
}

#[test]
fn test_last_on_empty_index_set() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            // No-op
        }
    }

    let mut index_set = TestMutableValues { values: Vec::new() };
    assert_eq!(index_set.last(), None);
}

