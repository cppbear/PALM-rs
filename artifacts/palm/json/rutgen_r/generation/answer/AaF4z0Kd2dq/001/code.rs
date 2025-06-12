// Answer 0

#[test]
fn test_next_valid_index() {
    struct SliceIterator {
        slice: Vec<u8>,
        index: usize,
    }

    impl SliceIterator {
        fn next(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                let ch = self.slice[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            })
        }
    }

    let mut iterator = SliceIterator {
        slice: vec![1, 2, 3, 4, 5],
        index: 0,
    };

    assert_eq!(iterator.next().unwrap(), Some(1));
    assert_eq!(iterator.next().unwrap(), Some(2));
    assert_eq!(iterator.next().unwrap(), Some(3));
    assert_eq!(iterator.next().unwrap(), Some(4));
    assert_eq!(iterator.next().unwrap(), Some(5));
    assert_eq!(iterator.next().unwrap(), None);
}

#[test]
fn test_next_empty_slice() {
    struct SliceIterator {
        slice: Vec<u8>,
        index: usize,
    }

    impl SliceIterator {
        fn next(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                let ch = self.slice[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            })
        }
    }

    let mut iterator = SliceIterator {
        slice: vec![],
        index: 0,
    };

    assert_eq!(iterator.next().unwrap(), None);
}

