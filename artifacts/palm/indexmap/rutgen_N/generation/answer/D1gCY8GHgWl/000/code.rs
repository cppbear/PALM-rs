// Answer 0

#[test]
fn test_as_slice() {
    struct TestIter<'a> {
        entries: &'a [i32],
        index: usize,
    }

    impl<'a> TestIter<'a> {
        pub fn new(entries: &'a [i32]) -> Self {
            TestIter { entries, index: 0 }
        }

        pub fn as_slice(&self) -> &'a [i32] {
            &self.entries[self.index..]
        }
    }

    let entries = [1, 2, 3, 4, 5];
    let iter = TestIter::new(&entries);

    let slice = iter.as_slice();
    assert_eq!(slice, [1, 2, 3, 4, 5]);

    // Simulate advancing the iterator
    let iter = TestIter { entries: &entries, index: 2 };
    let slice = iter.as_slice();
    assert_eq!(slice, [3, 4, 5]);
}

#[test]
fn test_as_slice_empty() {
    struct TestIter<'a> {
        entries: &'a [i32],
        index: usize,
    }

    impl<'a> TestIter<'a> {
        pub fn new(entries: &'a [i32]) -> Self {
            TestIter { entries, index: 0 }
        }

        pub fn as_slice(&self) -> &'a [i32] {
            &self.entries[self.index..]
        }
    }

    let entries: [i32; 0] = [];
    let iter = TestIter::new(&entries);

    let slice = iter.as_slice();
    assert_eq!(slice, []);
}

