// Answer 0

#[test]
fn test_next_at_end_of_slice() {
    struct TestSlice {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestSlice {
        fn next(&mut self) -> Result<Option<u8>, std::io::Error> {
            Ok(if self.index < self.slice.len() {
                let ch = self.slice[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            })
        }
    }

    let mut test_instance = TestSlice {
        slice: vec![1, 2, 3],
        index: 3, // Set index to the length of the slice to ensure boundary condition
    };

    let result = test_instance.next();
    assert_eq!(result, Ok(None)); // The expected outcome when index == slice.len()
}

#[test]
fn test_next_empty_slice() {
    struct TestSlice {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestSlice {
        fn next(&mut self) -> Result<Option<u8>, std::io::Error> {
            Ok(if self.index < self.slice.len() {
                let ch = self.slice[self.index];
                self.index += 1;
                Some(ch)
            } else {
                None
            })
        }
    }

    let mut test_instance = TestSlice {
        slice: vec![], // Empty slice
        index: 0,     // Starting index
    };

    let result = test_instance.next();
    assert_eq!(result, Ok(None)); // Expected outcome for an empty slice
}

