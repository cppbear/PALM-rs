// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
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
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    index_set.push(1);
    index_set.push(2);
    index_set.push(3);

    assert_eq!(index_set.swap_remove_index(1), Some(2));
    assert_eq!(index_set.as_slice().to_vec(), vec![1, 3]);
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
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
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    index_set.push(1);
    index_set.push(2);
    index_set.push(3);

    assert_eq!(index_set.swap_remove_index(5), None);
}

#[test]
fn test_swap_remove_index_empty() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
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
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };

    assert_eq!(index_set.swap_remove_index(0), None);
}

