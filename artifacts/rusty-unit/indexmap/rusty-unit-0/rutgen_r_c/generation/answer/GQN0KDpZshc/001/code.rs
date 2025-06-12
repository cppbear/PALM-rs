// Answer 0

#[test]
fn test_first_non_empty_slice() {
    struct TestSlice {
        entries: [Bucket<i32>; 3],
    }

    impl TestSlice {
        fn new() -> Self {
            TestSlice {
                entries: [
                    Bucket { hash: Default::default(), key: 1, value: 10 },
                    Bucket { hash: Default::default(), key: 2, value: 20 },
                    Bucket { hash: Default::default(), key: 3, value: 30 },
                ],
            }
        }
        
        fn as_slice(&self) -> Slice<i32> {
            Slice { entries: self.entries }
        }
    }

    let slice = TestSlice::new().as_slice();
    assert_eq!(slice.first(), Some(&1));
}

#[test]
fn test_first_empty_slice() {
    struct TestSlice {
        entries: [Bucket<i32>; 0],
    }

    impl TestSlice {
        fn new() -> Self {
            TestSlice { entries: [] }
        }
        
        fn as_slice(&self) -> Slice<i32> {
            Slice { entries: self.entries }
        }
    }

    let slice = TestSlice::new().as_slice();
    assert_eq!(slice.first(), None);
}

