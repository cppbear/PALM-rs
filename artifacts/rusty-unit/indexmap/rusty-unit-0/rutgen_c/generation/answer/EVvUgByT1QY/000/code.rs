// Answer 0

#[test]
fn test_get_index_mut_valid_index() {
    struct TestMutableKeys {
        entries: Vec<Bucket<i32, String>>,
    }

    impl private::Sealed for TestMutableKeys {}

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(
            &mut self,
            _key: &Q,
        ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            None // Not implemented for this test case
        }

        fn get_index_mut2(
            &mut self,
            index: usize,
        ) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.entries.get_mut(index).map(|bucket| (&mut bucket.key, &mut bucket.value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!() // Not required for this test case
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool,
        {
            unimplemented!() // Not required for this test case
        }
    }

    let mut map = TestMutableKeys {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "two".to_string() },
        ],
    };

    assert_eq!(map.get_index_mut2(0).map(|(k, v)| (*k, v.clone())), Some((1, "one".to_string())));
    assert_eq!(map.get_index_mut2(1).map(|(k, v)| (*k, v.clone())), Some((2, "two".to_string())));
}

#[test]
fn test_get_index_mut_invalid_index() {
    struct TestMutableKeys {
        entries: Vec<Bucket<i32, String>>,
    }

    impl private::Sealed for TestMutableKeys {}

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(
            &mut self,
            _key: &Q,
        ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            None // Not implemented for this test case
        }

        fn get_index_mut2(
            &mut self,
            index: usize,
        ) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.entries.get_mut(index).map(|bucket| (&mut bucket.key, &mut bucket.value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!() // Not required for this test case
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool,
        {
            unimplemented!() // Not required for this test case
        }
    }

    let mut map = TestMutableKeys {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "two".to_string() },
        ],
    };

    assert_eq!(map.get_index_mut2(2), None);
}

#[test]
fn test_get_index_mut_empty() {
    struct TestMutableKeys {
        entries: Vec<Bucket<i32, String>>,
    }

    impl private::Sealed for TestMutableKeys {}

    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;

        fn get_full_mut2<Q>(
            &mut self,
            _key: &Q,
        ) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key>,
        {
            None // Not implemented for this test case
        }

        fn get_index_mut2(
            &mut self,
            index: usize,
        ) -> Option<(&mut Self::Key, &mut Self::Value)> {
            self.entries.get_mut(index).map(|bucket| (&mut bucket.key, &mut bucket.value))
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!() // Not required for this test case
        }

        fn retain2<F>(&mut self, _keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool,
        {
            unimplemented!() // Not required for this test case
        }
    }

    let mut map = TestMutableKeys {
        entries: vec![],
    };

    assert_eq!(map.get_index_mut2(0), None);
}

