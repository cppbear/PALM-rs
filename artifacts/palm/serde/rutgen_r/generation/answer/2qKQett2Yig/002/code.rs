// Answer 0

fn test_end_function_with_remaining_zero() {
    struct SeqDeserializer {
        iter: std::vec::IntoIter<i32>,
        count: usize,
    }

    impl SeqDeserializer {
        pub fn new(data: Vec<i32>) -> Self {
            Self {
                iter: data.into_iter(),
                count: data.len(),
            }
        }

        pub fn end(self) -> Result<(), String> {
            let remaining = self.iter.count();
            if remaining == 0 {
                Ok(())
            } else {
                Err(format!("invalid_length: {}, expected: {:?}", self.count + remaining, ExpectedInSeq(self.count)))
            }
        }
    }

    struct ExpectedInSeq(usize);

    let deserializer = SeqDeserializer::new(vec![]);
    let result = deserializer.end();
    assert_eq!(result, Ok(()));
}

fn test_end_function_with_one_remaining() {
    struct SeqDeserializer {
        iter: std::vec::IntoIter<i32>,
        count: usize,
    }

    impl SeqDeserializer {
        pub fn new(data: Vec<i32>) -> Self {
            Self {
                iter: data.into_iter(),
                count: data.len(),
            }
        }

        pub fn end(self) -> Result<(), String> {
            let remaining = self.iter.count();
            if remaining == 0 {
                Ok(())
            } else {
                Err(format!("invalid_length: {}, expected: {:?}", self.count + remaining, ExpectedInSeq(self.count)))
            }
        }
    }

    struct ExpectedInSeq(usize);

    let deserializer = SeqDeserializer::new(vec![1]);
    let result = deserializer.end();
    assert_eq!(result, Err(format!("invalid_length: 1, expected: {:?}", ExpectedInSeq(1))));
}

