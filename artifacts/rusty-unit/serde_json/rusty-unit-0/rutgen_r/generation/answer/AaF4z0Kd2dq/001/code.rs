// Answer 0

#[derive(Debug)]
struct SliceIterator {
    slice: Vec<u8>,
    index: usize,
}

impl SliceIterator {
    fn next(&mut self) -> Result<Option<u8>> {
        Ok(if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        })
    }
}

#[test]
fn test_next_with_valid_index() {
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
fn test_next_with_non_zero_initial_index() {
    let mut iterator = SliceIterator {
        slice: vec![10, 20, 30],
        index: 1,
    };

    assert_eq!(iterator.next().unwrap(), Some(20));
    assert_eq!(iterator.next().unwrap(), Some(30));
    assert_eq!(iterator.next().unwrap(), None);
}

#[test]
fn test_next_with_empty_slice() {
    let mut iterator = SliceIterator {
        slice: vec![],
        index: 0,
    };

    assert_eq!(iterator.next().unwrap(), None);
}

#[test]
fn test_next_with_exceeding_index() {
    let mut iterator = SliceIterator {
        slice: vec![100, 200, 300],
        index: 3,
    };

    assert_eq!(iterator.next().unwrap(), None);
}

