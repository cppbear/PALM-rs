// Answer 0

#[test]
fn test_peek_valid_index() {
    struct SliceReader {
        slice: Vec<u8>,
        index: usize,
    }

    impl SliceReader {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut reader = SliceReader {
        slice: vec![1, 2, 3, 4, 5],
        index: 2,
    };
    
    let result = reader.peek().unwrap();
    assert_eq!(result, Some(3));
}

#[test]
fn test_peek_first_element() {
    struct SliceReader {
        slice: Vec<u8>,
        index: usize,
    }

    impl SliceReader {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut reader = SliceReader {
        slice: vec![10, 20, 30],
        index: 0,
    };
    
    let result = reader.peek().unwrap();
    assert_eq!(result, Some(10));
}

#[test]
fn test_peek_last_element() {
    struct SliceReader {
        slice: Vec<u8>,
        index: usize,
    }

    impl SliceReader {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut reader = SliceReader {
        slice: vec![100, 200, 300],
        index: 2,
    };
    
    let result = reader.peek().unwrap();
    assert_eq!(result, Some(300));
}

#[test]
fn test_peek_out_of_bounds() {
    struct SliceReader {
        slice: Vec<u8>,
        index: usize,
    }

    impl SliceReader {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut reader = SliceReader {
        slice: vec![5, 10, 15],
        index: 3,
    };
    
    let result = reader.peek().unwrap();
    assert_eq!(result, None);
}

