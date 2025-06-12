// Answer 0

#[test]
fn test_into_boxed_slice_empty_set() {
    struct TestSet {
        entries: Vec<i32>,
    }

    impl TestSet {
        fn into_entries(self) -> Vec<i32> {
            self.entries
        }
        
        fn into_boxed_slice(self) -> Box<[i32]> {
            self.into_entries().into_boxed_slice()
        }
    }

    let set = TestSet { entries: vec![] };
    let boxed_slice = set.into_boxed_slice();
    assert_eq!(boxed_slice.len(), 0);
}

#[test]
fn test_into_boxed_slice_non_empty_set() {
    struct TestSet {
        entries: Vec<i32>,
    }

    impl TestSet {
        fn into_entries(self) -> Vec<i32> {
            self.entries
        }
        
        fn into_boxed_slice(self) -> Box<[i32]> {
            self.into_entries().into_boxed_slice()
        }
    }

    let set = TestSet { entries: vec![1, 2, 3] };
    let boxed_slice = set.into_boxed_slice();
    assert_eq!(boxed_slice.len(), 3);
    assert_eq!(&*boxed_slice, &[1, 2, 3]);
}

