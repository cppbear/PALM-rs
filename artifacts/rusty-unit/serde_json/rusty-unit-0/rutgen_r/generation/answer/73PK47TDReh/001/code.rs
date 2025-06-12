// Answer 0

fn test_peek_with_valid_index() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut test_instance = TestStruct {
        slice: vec![1, 2, 3],
        index: 1,
    };

    assert_eq!(test_instance.peek(), Ok(Some(2)));
}

fn test_peek_with_index_at_bound() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut test_instance = TestStruct {
        slice: vec![10, 20, 30],
        index: 2,
    };

    assert_eq!(test_instance.peek(), Ok(Some(30)));
}

fn test_peek_with_index_exceeding_length() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
    }

    let mut test_instance = TestStruct {
        slice: vec![5, 15, 25],
        index: 3,
    };

    assert_eq!(test_instance.peek(), Ok(None));
}

