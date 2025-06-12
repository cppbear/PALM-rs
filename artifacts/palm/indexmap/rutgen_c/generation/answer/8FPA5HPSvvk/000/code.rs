// Answer 0

#[test]
fn test_last_mut_empty() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }
    
    impl MutableKeys for TestMap {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(
            &mut self,
            _key: &Q,
        ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(
            &mut self,
            _index: usize,
        ) -> Option<(&mut Self::Key, &mut Self::Value)> {
            None
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
    };

    assert_eq!(map.last_mut(), None);
}

#[test]
fn test_last_mut_one_element() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }
    
    impl MutableKeys for TestMap {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(
            &mut self,
            _key: &Q,
        ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(
            &mut self,
            index: usize,
        ) -> Option<(&mut Self::Key, &mut Self::Value)> {
            Some((&mut self.core.entries[index].key, &mut self.core.entries[index].value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries(vec![Bucket { hash: HashValue::new(), key: 1, value: 10 }]),
        },
    };

    let last = map.last_mut();
    assert!(last.is_some());
    let (key, value) = last.unwrap();
    assert_eq!(*key, 1);
    *value += 5;
    assert_eq!(map.core.entries[0].value, 15);
}

#[test]
fn test_last_mut_multiple_elements() {
    struct TestMap {
        core: IndexMapCore<i32, i32>,
    }
    
    impl MutableKeys for TestMap {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(
            &mut self,
            _key: &Q,
        ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(
            &mut self,
            index: usize,
        ) -> Option<(&mut Self::Key, &mut Self::Value)> {
            Some((&mut self.core.entries[index].key, &mut self.core.entries[index].value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            unimplemented!()
        }
    }

    let mut map = TestMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries(vec![Bucket { hash: HashValue::new(), key: 1, value: 10 }, Bucket { hash: HashValue::new(), key: 2, value: 20 }]),
        },
    };

    let last = map.last_mut();
    assert!(last.is_some());
    let (key, value) = last.unwrap();
    assert_eq!(*key, 2);
    *value += 10;
    assert_eq!(map.core.entries[1].value, 30);
}

