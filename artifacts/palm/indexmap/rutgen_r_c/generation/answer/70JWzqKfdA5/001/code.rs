// Answer 0

#[test]
fn test_pop_empty() {
    struct TestMutableValues;

    impl private::Sealed for TestMutableValues {}

    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, _: usize) -> Option<&mut Self::Value> {
            None
        }

        fn retain2<F>(&mut self, _: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {}
    }
    
    let mut index_set: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    assert_eq!(index_set.pop(), None);
}

#[test]
fn test_pop_single_element() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}

    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)> 
        where 
            Q: ?Sized + Hash + Equivalent<Self::Value> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {}
    }

    let mut index_set: IndexSet<i32, RandomState> = IndexSet { 
        map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() }
    };
    
    index_set.map.insert(1, ());
    assert_eq!(index_set.pop(), Some(1));
    assert_eq!(index_set.pop(), None);
}

#[test]
fn test_pop_multiple_elements() {
    struct TestMutableValues {
        values: Vec<i32>,
    }

    impl private::Sealed for TestMutableValues {}

    impl MutableValues for TestMutableValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _: &Q) -> Option<(usize, &mut Self::Value)> 
        where 
            Q: ?Sized + Hash + Equivalent<Self::Value> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.values.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {}
    }

    let mut index_set: IndexSet<i32, RandomState> = IndexSet { 
        map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() }
    };
    
    index_set.map.insert(1, ());
    index_set.map.insert(2, ());
    index_set.map.insert(3, ());
    
    assert_eq!(index_set.pop(), Some(3));
    assert_eq!(index_set.pop(), Some(2));
    assert_eq!(index_set.pop(), Some(1));
    assert_eq!(index_set.pop(), None);
}

