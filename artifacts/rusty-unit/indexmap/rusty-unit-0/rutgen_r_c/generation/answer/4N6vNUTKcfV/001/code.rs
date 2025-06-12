// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestMutableKeys;
    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;
        
        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            None
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
        }
    }

    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestIndexMap {
        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn as_slice(&self) -> &Slice<i32, String> {
            Slice::from_slice(self.as_entries())
        }
    }

    let map = TestIndexMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: String::from("one") },
            Bucket { hash: HashValue::default(), key: 2, value: String::from("two") },
        ],
    };

    let slice = map.as_slice();
    assert_eq!(slice.entries.len(), 2);
}

#[test]
fn test_as_slice_empty() {
    struct TestMutableKeys;
    impl MutableKeys for TestMutableKeys {
        type Key = i32;
        type Value = String;
        
        fn get_full_mut2<Q>(&mut self, key: &Q) -> Option<(usize, &mut Self::Key, &mut Self::Value)>
        where
            Q: ?Sized + Hash + Equivalent<Self::Key> {
            None
        }

        fn get_index_mut2(&mut self, index: usize) -> Option<(&mut Self::Key, &mut Self::Value)> {
            None
        }

        fn iter_mut2(&mut self) -> IterMut2<'_, Self::Key, Self::Value> {
            unimplemented!()
        }

        fn retain2<F>(&mut self, keep: F)
        where
            F: FnMut(&mut Self::Key, &mut Self::Value) -> bool {
        }
    }

    struct TestIndexMap {
        entries: Vec<Bucket<i32, String>>,
    }

    impl TestIndexMap {
        fn as_entries(&self) -> &[Bucket<i32, String>] {
            &self.entries
        }

        fn as_slice(&self) -> &Slice<i32, String> {
            Slice::from_slice(self.as_entries())
        }
    }

    let map = TestIndexMap { entries: Vec::new() };

    let slice = map.as_slice();
    assert_eq!(slice.entries.len(), 0);
}

