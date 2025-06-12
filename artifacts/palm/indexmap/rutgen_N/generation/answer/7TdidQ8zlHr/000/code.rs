// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct MockIterator {
        entries: Vec<(i32, i32)>,
        index: usize,
    }
    
    impl MockIterator {
        pub fn new(entries: Vec<(i32, i32)>) -> Self {
            MockIterator { entries, index: 0 }
        }
        
        pub fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries[self.index..]
        }
    }
    
    let iter = MockIterator::new(vec![(1, 10), (2, 20), (3, 30)]);
    let slice = iter.as_slice();
    assert_eq!(slice, &[(1, 10), (2, 20), (3, 30)]);
}

#[test]
fn test_as_slice_empty() {
    struct MockIterator {
        entries: Vec<(i32, i32)>,
        index: usize,
    }
    
    impl MockIterator {
        pub fn new(entries: Vec<(i32, i32)>) -> Self {
            MockIterator { entries, index: 0 }
        }
        
        pub fn as_slice(&self) -> &[(i32, i32)] {
            &self.entries[self.index..]
        }
    }

    let iter = MockIterator::new(vec![]);
    let slice = iter.as_slice();
    assert_eq!(slice, &[]);
}

