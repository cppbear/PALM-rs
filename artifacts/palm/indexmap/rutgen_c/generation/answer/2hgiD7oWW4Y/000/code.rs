// Answer 0

#[test]
fn test_get_mut_existing_key() {
    struct TestMutableKeys {
        entries: Vec<(i32, String)>,
    }
    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>
        {
            None
        }
        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.entries.get_mut(index).map(|(k, v)| (k, v))
        }
        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            todo!()
        }
        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool
        {
            todo!()
        }
    }

    let mut map = IndexMap::<i32, String, RandomState>::default();
    map.insert(1, "one".to_string());
    
    let value = map.get_mut(&1).unwrap();
    *value = "modified".to_string();
    assert_eq!(map.get(&1), Some(&"modified".to_string()));
}

#[test]
fn test_get_mut_non_existent_key() {
    struct TestMutableKeys {
        entries: Vec<(i32, String)>,
    }
    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>
        {
            None
        }
        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.entries.get_mut(index).map(|(k, v)| (k, v))
        }
        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            todo!()
        }
        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool
        {
            todo!()
        }
    }
    
    let mut map = IndexMap::<i32, String, RandomState>::default();

    assert!(map.get_mut(&1).is_none());
}

