// Answer 0

#[test]
fn test_retain_with_all_true() {
    struct TestMutableKeys {
        data: Vec<(i32, String)>,
    }

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
            where Q: ?Sized + Hash + Equivalent<Self::Key> {
                None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.data.get_mut(index).map(|(k, v)| (k, v))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            IterMut2::new(&mut self.data)
        }

        fn retain2<F>(&mut self, keep: F)
            where F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
                self.data.retain(|(k, v)| keep(k, v));
        }
    }

    let mut map = TestMutableKeys {
        data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
    };

    map.retain2(|_k, _v| true);
    assert_eq!(map.data.len(), 3);
}

#[test]
fn test_retain_with_some_false() {
    struct TestMutableKeys {
        data: Vec<(i32, String)>,
    }

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
            where Q: ?Sized + Hash + Equivalent<Self::Key> {
                None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.data.get_mut(index).map(|(k, v)| (k, v))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            IterMut2::new(&mut self.data)
        }

        fn retain2<F>(&mut self, keep: F)
            where F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
                self.data.retain(|(k, v)| keep(k, v));
        }
    }

    let mut map = TestMutableKeys {
        data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
    };

    map.retain2(|_k, v| v != "two");
    assert_eq!(map.data.len(), 2);
    assert!(!map.data.iter().any(|(_, v)| v == "two"));
}

#[test]
fn test_retain_with_all_false() {
    struct TestMutableKeys {
        data: Vec<(i32, String)>,
    }

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
            where Q: ?Sized + Hash + Equivalent<Self::Key> {
                None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.data.get_mut(index).map(|(k, v)| (k, v))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            IterMut2::new(&mut self.data)
        }

        fn retain2<F>(&mut self, keep: F)
            where F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
                self.data.retain(|(k, v)| keep(k, v));
        }
    }

    let mut map = TestMutableKeys {
        data: vec![(1, "one".to_string()), (2, "two".to_string()), (3, "three".to_string())],
    };

    map.retain2(|_k, _v| false);
    assert_eq!(map.data.len(), 0);
}

#[test]
fn test_retain_with_empty() {
    struct TestMutableKeys {
        data: Vec<(i32, String)>,
    }

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(&mut self, _key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
            where Q: ?Sized + Hash + Equivalent<Self::Key> {
                None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.data.get_mut(index).map(|(k, v)| (k, v))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            IterMut2::new(&mut self.data)
        }

        fn retain2<F>(&mut self, keep: F)
            where F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
                self.data.retain(|(k, v)| keep(k, v));
        }
    }

    let mut map = TestMutableKeys { data: vec![] };

    map.retain2(|_k, _v| true);
    assert_eq!(map.data.len(), 0);
}

