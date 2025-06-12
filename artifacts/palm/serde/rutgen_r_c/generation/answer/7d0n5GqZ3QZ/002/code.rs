// Answer 0

#[test]
fn test_visit_seq_valid() {
    struct MockSeqAccess {
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value: T = serde::de::Deserialize::deserialize(self.values[self.index].into())?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq_access = MockSeqAccess { values: vec![5], index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an integer", phantom: PhantomData };
    let result = visitor.visit_seq(seq_access);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_visit_seq_empty() {
    struct MockSeqAccess {
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value: T = serde::de::Deserialize::deserialize(self.values[self.index].into())?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq_access = MockSeqAccess { values: vec![], index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an integer", phantom: PhantomData };
    let result = visitor.visit_seq(seq_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_seq_invalid_type() {
    struct MockSeqAccess {
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for MockSeqAccess {
        type Error = serde::de::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'de>,
        {
            Ok(Some(42))  // Simulating an incorrect type return
        }
    }

    let seq_access = MockSeqAccess { values: vec![42], index: 0 };
    let visitor = RangeToVisitor::<i32> { expecting: "an integer", phantom: PhantomData };
    let result = visitor.visit_seq(seq_access);
    assert!(result.is_err());
}

