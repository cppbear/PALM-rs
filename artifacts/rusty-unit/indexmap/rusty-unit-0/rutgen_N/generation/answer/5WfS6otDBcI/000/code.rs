// Answer 0

#[test]
fn test_into_boxed_slice_empty() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn into_entries(self) -> TestMapEntries {
            TestMapEntries { entries: self.entries }
        }
    }

    struct TestMapEntries {
        entries: Vec<(i32, i32)>,
    }

    impl TestMapEntries {
        fn into_boxed_slice(self) -> Box<[(i32, i32)]> {
            self.entries.into_boxed_slice()
        }
    }

    struct Slice<K, V> {
        data: Box<[(K, V)]>,
    }

    impl<K, V> Slice<K, V> {
        fn from_boxed(data: Box<[(K, V)]>) -> Box<Slice<K, V>> {
            Box::new(Slice { data })
        }
    }

    let map = TestMap { entries: vec![] };
    let boxed_slice: Box<Slice<i32, i32>> = map.into_entries().into_boxed_slice();
    assert!(boxed_slice.data.is_empty());
}

#[test]
fn test_into_boxed_slice_non_empty() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        fn into_entries(self) -> TestMapEntries {
            TestMapEntries { entries: self.entries }
        }
    }

    struct TestMapEntries {
        entries: Vec<(i32, i32)>,
    }

    impl TestMapEntries {
        fn into_boxed_slice(self) -> Box<[(i32, i32)]> {
            self.entries.into_boxed_slice()
        }
    }

    struct Slice<K, V> {
        data: Box<[(K, V)]>,
    }

    impl<K, V> Slice<K, V> {
        fn from_boxed(data: Box<[(K, V)]>) -> Box<Slice<K, V>> {
            Box::new(Slice { data })
        }
    }

    let map = TestMap { entries: vec![(1, 2), (3, 4)] };
    let boxed_slice: Box<Slice<i32, i32>> = map.into_entries().into_boxed_slice();
    assert_eq!(boxed_slice.data.len(), 2);
    assert_eq!(boxed_slice.data[0], (1, 2));
    assert_eq!(boxed_slice.data[1], (3, 4));
}

