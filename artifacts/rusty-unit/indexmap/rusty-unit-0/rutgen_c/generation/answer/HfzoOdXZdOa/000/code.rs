// Answer 0

#[test]
fn test_truncate_with_exact_length() {
    struct TestValues {
        data: Vec<i32>,
    }

    impl MutableValues for TestValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.data.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool {
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    index_set.map.core = IndexMap::new();
    index_set.map.core.push(1);
    index_set.map.core.push(2);
    index_set.map.core.push(3);

    index_set.truncate(2);
    assert_eq!(index_set.len(), 2);
}

#[test]
fn test_truncate_with_exceeding_length() {
    struct TestValues {
        data: Vec<i32>,
    }

    impl MutableValues for TestValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.data.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool {
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    index_set.map.core = IndexMap::new();
    index_set.map.core.push(1);
    index_set.map.core.push(2);

    index_set.truncate(5);
    assert_eq!(index_set.len(), 2);
}

#[test]
fn test_truncate_with_zero_length() {
    struct TestValues {
        data: Vec<i32>,
    }

    impl MutableValues for TestValues {
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Value> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<&mut Self::Value> {
            self.data.get_mut(index)
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Value) -> bool {
        }
    }

    let mut index_set = IndexSet::with_capacity_and_hasher(5, RandomState::new());
    index_set.map.core = IndexMap::new();
    index_set.map.core.push(1);
    index_set.map.core.push(2);

    index_set.truncate(0);
    assert_eq!(index_set.len(), 0);
}

