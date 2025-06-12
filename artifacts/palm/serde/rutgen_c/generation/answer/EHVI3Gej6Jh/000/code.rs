// Answer 0

#[test]
fn test_visit_seq_with_valid_input() {
    struct TestSeqAccess {
        data: Vec<i64>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = MockError;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value as T))
            } else {
                Ok(None)
            }
        }
    }

    struct MockError;

    impl Error for MockError {
        fn invalid_length(length: usize, visitor: &dyn Visitor<'_>) -> Self {
            MockError
        }
    }

    let seq_access = TestSeqAccess { data: vec![1, 2], index: 0 };
    let visitor = RangeVisitor::<i64> {
        expecting: "a sequence of two elements",
        phantom: std::marker::PhantomData,
    };
    let result: Result<(i64, i64), MockError> = visitor.visit_seq(seq_access);
    assert_eq!(result.unwrap(), (1, 2));
}

#[test]
fn test_visit_seq_with_missing_start() {
    struct TestSeqAccess {
        data: Vec<i64>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = MockError;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value as T))
            } else {
                Ok(None)
            }
        }
    }

    let seq_access = TestSeqAccess { data: vec![1], index: 0 };
    let visitor = RangeVisitor::<i64> {
        expecting: "a sequence of two elements",
        phantom: std::marker::PhantomData,
    };
    let result: Result<(i64, i64), MockError> = visitor.visit_seq(seq_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_seq_with_missing_end() {
    struct TestSeqAccess {
        data: Vec<i64>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = MockError;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.data.len() {
                let value = self.data[self.index];
                self.index += 1;
                Ok(Some(value as T))
            } else {
                Ok(None)
            }
        }
    }

    let seq_access = TestSeqAccess { data: vec![1, 2, 3], index: 0 };
    let visitor = RangeVisitor::<i64> {
        expecting: "a sequence of two elements",
        phantom: std::marker::PhantomData,
    };
    let result: Result<(i64, i64), MockError> = visitor.visit_seq(seq_access);
    assert!(result.is_err());
}

