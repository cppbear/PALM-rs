// Answer 0

#[test]
fn test_sort_by() {
    struct TestMutableValues {
        data: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
    
    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.data.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            self.data.retain(keep);
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::from_vec(vec![3, 1, 2]),
            hash_builder: RandomState::new(),
        },
    };

    index_set.sort_by(|a, b| a.cmp(b));
    assert_eq!(index_set.as_slice(), &Slice::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_sort_by_empty() {
    struct TestMutableValues {
        data: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}
    
    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.data.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
            self.data.retain(keep);
        }
    }

    let mut index_set = IndexSet {
        map: IndexMap {
            core: IndexMapCore::from_vec(Vec::new()),
            hash_builder: RandomState::new(),
        },
    };

    index_set.sort_by(|a, b| a.cmp(b));
    assert!(index_set.as_slice().is_empty());
}

