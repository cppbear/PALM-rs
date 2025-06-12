// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestBucket {
        key: u32,
        value: (),
    }
    
    struct TestIndexSet {
        entries: Vec<TestBucket>,
    }

    impl TestIndexSet {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }
        
        pub fn as_entries(&self) -> &[TestBucket] {
            &self.entries
        }

        pub fn push(&mut self, key: u32) {
            self.entries.push(TestBucket { key, value: () });
        }
    }

    let mut index_set = TestIndexSet::new();
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);

    assert_eq!(index_set.get_index(0).map(|b| b.key), Some(1));
    assert_eq!(index_set.get_index(1).map(|b| b.key), Some(2));
    assert_eq!(index_set.get_index(2).map(|b| b.key), Some(3));
}

#[test]
fn test_get_index_invalid() {
    struct TestBucket {
        key: u32,
        value: (),
    }
    
    struct TestIndexSet {
        entries: Vec<TestBucket>,
    }

    impl TestIndexSet {
        pub fn new() -> Self {
            Self { entries: Vec::new() }
        }
        
        pub fn as_entries(&self) -> &[TestBucket] {
            &self.entries
        }

        pub fn push(&mut self, key: u32) {
            self.entries.push(TestBucket { key, value: () });
        }
    }

    let mut index_set = TestIndexSet::new();
    index_set.push(1);
    index_set.push(2);
    index_set.push(3);

    assert_eq!(index_set.get_index(3), None);
    assert_eq!(index_set.get_index(usize::MAX), None);
}

