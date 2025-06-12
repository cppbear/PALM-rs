// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestSet {
        entries: Vec<Bucket<i32>>,
    }
    
    impl TestSet {
        fn new() -> Self {
            TestSet { entries: vec![] }
        }

        fn as_entries(&self) -> &[Bucket<i32>] {
            &self.entries
        }
    }
    
    let mut set = TestSet::new();
    set.entries.push(Bucket { hash: 0, key: 1, value: () });
    set.entries.push(Bucket { hash: 0, key: 2, value: () });
    
    let slice = set.as_slice();
    assert_eq!(slice.entries.len(), 2);
}

#[test]
fn test_as_slice_empty() {
    struct TestSet {
        entries: Vec<Bucket<i32>>,
    }
    
    impl TestSet {
        fn new() -> Self {
            TestSet { entries: vec![] }
        }

        fn as_entries(&self) -> &[Bucket<i32>] {
            &self.entries
        }
    }
    
    let set = TestSet::new();
    let slice = set.as_slice();
    assert_eq!(slice.entries.len(), 0);
}

