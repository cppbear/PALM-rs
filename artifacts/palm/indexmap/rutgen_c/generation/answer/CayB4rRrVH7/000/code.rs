// Answer 0

#[test]
fn test_into_boxed_slice_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }
        
        fn into_boxed_slice(self) -> Box<Slice<i32>> {
            Slice::from_boxed(self.into_entries().into_boxed_slice())
        }
    }

    let test_map = TestMap { entries: Vec::new() };
    let boxed_slice = test_map.into_boxed_slice();
    assert_eq!(boxed_slice.entries.len(), 0);
}

#[test]
fn test_into_boxed_slice_non_empty() {
    struct TestMap {
        entries: Vec<Bucket<i32>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }
        
        fn into_boxed_slice(self) -> Box<Slice<i32>> {
            Slice::from_boxed(self.into_entries().into_boxed_slice())
        }
    }

    let test_map = TestMap { 
        entries: vec![
            Bucket { hash: 1, key: 1, value: () },
            Bucket { hash: 2, key: 2, value: () },
        ] 
    };
    let boxed_slice = test_map.into_boxed_slice();
    assert_eq!(boxed_slice.entries.len(), 2);
}

#[test]
#[should_panic]
fn test_into_boxed_slice_panic_on_drop() {
    struct TestMap {
        entries: Vec<Bucket<i32>>,
    }

    impl TestMap {
        fn into_entries(self) -> Vec<Bucket<i32>> {
            self.entries
        }
        
        fn into_boxed_slice(self) -> Box<Slice<i32>> {
            Slice::from_boxed(self.into_entries().into_boxed_slice())
        }
    }

    let _test_map = TestMap { 
        entries: vec![
            Bucket { hash: 1, key: 1, value: () },
            Bucket { hash: 2, key: 2, value: () },
        ] 
    };
    // This will drop the inner hash table, potentially causing issues if there were references held.
    let _boxed_slice = _test_map.into_boxed_slice();
}

