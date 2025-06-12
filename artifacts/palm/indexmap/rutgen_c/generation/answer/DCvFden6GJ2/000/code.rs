// Answer 0

#[test]
fn test_is_superset_true() {
    use std::collections::hash_map::RandomState;

    struct DummyValues;

    impl MutableValues for DummyValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, _index: usize) -> Option<&mut Self::Value> {
            None
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
        }
    }

    let map1 = IndexMap::<i32, (), RandomState> {
        core: IndexMapCore::new(),
        hash_builder: RandomState::new(),
    };
    let set1 = IndexSet { map: map1 };

    let map2 = IndexMap::<i32, (), RandomState> {
        core: IndexMapCore::new(),
        hash_builder: RandomState::new(),
    };
    let set2 = IndexSet { map: map2 };

    assert!(set1.is_superset(&set2));
}

#[test]
fn test_is_superset_false() {
    use std::collections::hash_map::RandomState;

    struct DummyValues;

    impl MutableValues for DummyValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, _value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value>,
        {
            None
        }

        fn get_index_mut2(&mut self, _index: usize) -> Option<&mut Self::Value> {
            None
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool,
        {
        }
    }

    let map1 = IndexMap::<i32, (), RandomState> {
        core: IndexMapCore::new(),
        hash_builder: RandomState::new(),
    };
    let set1 = IndexSet { map: map1 };

    let map2 = IndexMap::<i32, (), RandomState> {
        core: IndexMapCore::new(),
        hash_builder: RandomState::new(),
    };
    let set2 = IndexSet { map: map2 };

    assert!(!set1.is_superset(&set2));
}

