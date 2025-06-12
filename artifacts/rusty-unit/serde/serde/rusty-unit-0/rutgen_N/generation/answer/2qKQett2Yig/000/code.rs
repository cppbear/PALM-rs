// Answer 0

#[test]
fn test_end_with_no_remaining_elements() {
    struct TestSeqDeserializer {
        count: usize,
        iter: Vec<usize>,
    }

    impl TestSeqDeserializer {
        fn new(count: usize, iter: Vec<usize>) -> Self {
            TestSeqDeserializer { count, iter }
        }
        
        pub fn end(self) -> Result<(), String> {
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(())
            } else {
                Err(format!(
                    "invalid_length({}, ExpectedInSeq({}))",
                    self.count + remaining,
                    self.count
                ))
            }
        }
    }

    let deserializer = TestSeqDeserializer::new(0, vec![]);
    let result = deserializer.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_with_remaining_elements() {
    struct TestSeqDeserializer {
        count: usize,
        iter: Vec<usize>,
    }

    impl TestSeqDeserializer {
        fn new(count: usize, iter: Vec<usize>) -> Self {
            TestSeqDeserializer { count, iter }
        }
        
        pub fn end(self) -> Result<(), String> {
            let remaining = self.iter.len();
            if remaining == 0 {
                Ok(())
            } else {
                Err(format!(
                    "invalid_length({}, ExpectedInSeq({}))",
                    self.count + remaining,
                    self.count
                ))
            }
        }
    }

    let deserializer = TestSeqDeserializer::new(0, vec![1, 2, 3]);
    let result = deserializer.end();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "invalid_length(3, ExpectedInSeq(0))");
}

