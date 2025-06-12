// Answer 0

#[test]
fn test_visit_seq_success() {
    struct TestSeq {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::Error;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.elements.len() {
                let value = seed.deserialize(self.elements[self.index])?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        elements: vec![1, 2],
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };
    
    let result: Result<(i32, i32), _> = visitor.visit_seq(seq);
    assert_eq!(result, Ok((1, 2)));
}

#[test]
fn test_visit_seq_missing_start() {
    struct TestSeq {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::Error;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.elements.len() {
                let value = seed.deserialize(self.elements[self.index])?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        elements: vec![1], // Only one element provided
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_seq(seq);
    assert!(result.is_err());
}

#[test]
fn test_visit_seq_missing_end() {
    struct TestSeq {
        elements: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeq {
        type Error = serde::de::Error;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.elements.len() {
                let value = seed.deserialize(self.elements[self.index])?;
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq {
        elements: vec![1, 2, 3], // 3 elements, will stop after 2
        index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_seq(seq);
    assert!(result.is_err());
}

