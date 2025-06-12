// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    struct TestMutableKeys {
        data: Vec<(i32, i32)>,
    }

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.data.get_mut(index).map(|(k, v)| (k, v))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            todo!()
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            todo!()
        }
    }

    let mut map = IndexMap::<i32, i32, ()> {
        core: IndexMapCore::new(),
        hash_builder: (),
    };

    map.push_entry(0, 1, 10);
    map.push_entry(0, 2, 20);
    map.push_entry(0, 3, 30);

    let removed = map.shift_remove_index(1);
    assert_eq!(removed, Some((2, 20)));
    assert_eq!(map.get_index(1), Some((&3, &30)));
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    struct TestMutableKeys {
        data: Vec<(i32, i32)>,
    }

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = i32;

        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.data.get_mut(index).map(|(k, v)| (k, v))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            todo!()
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
            todo!()
        }
    }

    let mut map = IndexMap::<i32, i32, ()> {
        core: IndexMapCore::new(),
        hash_builder: (),
    };

    map.push_entry(0, 1, 10);
    map.push_entry(0, 2, 20);
    
    let removed = map.shift_remove_index(2);
    assert_eq!(removed, None);
}

