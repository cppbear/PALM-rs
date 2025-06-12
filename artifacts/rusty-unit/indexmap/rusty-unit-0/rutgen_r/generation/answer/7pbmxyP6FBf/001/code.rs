// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestIter<'a> {
        data: &'a [(i32, i32)],
        index: usize,
    }

    impl<'a> TestIter<'a> {
        fn new(data: &'a [(i32, i32)]) -> Self {
            Self { data, index: 0 }
        }

        fn as_slice(&self) -> &'a [(i32, i32)] {
            &self.data[self.index..]
        }
    }

    struct TestMap<'a> {
        iter: TestIter<'a>,
    }

    impl<'a> TestMap<'a> {
        fn new(data: &'a [(i32, i32)]) -> Self {
            Self { iter: TestIter::new(data) }
        }

        fn as_slice(&self) -> &[&(i32, i32)] {
            let slice = self.iter.as_slice();
            slice
        }
    }

    let data = [(1, 2), (3, 4), (5, 6)];
    let map = TestMap::new(&data);
    let result = map.as_slice();
    assert_eq!(result, &[(1, 2), (3, 4), (5, 6)]);
}

#[test]
fn test_as_slice_empty() {
    struct TestIter<'a> {
        data: &'a [(i32, i32)],
        index: usize,
    }

    impl<'a> TestIter<'a> {
        fn new(data: &'a [(i32, i32)]) -> Self {
            Self { data, index: 0 }
        }

        fn as_slice(&self) -> &'a [(i32, i32)] {
            &self.data[self.index..]
        }
    }

    struct TestMap<'a> {
        iter: TestIter<'a>,
    }

    impl<'a> TestMap<'a> {
        fn new(data: &'a [(i32, i32)]) -> Self {
            Self { iter: TestIter::new(data) }
        }

        fn as_slice(&self) -> &[&(i32, i32)] {
            let slice = self.iter.as_slice();
            slice
        }
    }

    let data: [(i32, i32); 0] = [];
    let map = TestMap::new(&data);
    let result = map.as_slice();
    assert_eq!(result.len(), 0);
}

