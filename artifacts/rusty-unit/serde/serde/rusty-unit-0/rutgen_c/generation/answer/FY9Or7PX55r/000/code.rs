// Answer 0

#[test]
fn test_next_element_some() {
    struct TestSeqAccess {
        values: Vec<i32>,
        index: usize,
    }
    
    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = std::convert::Infallible;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index].clone();
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let mut seq_access = TestSeqAccess { values: vec![1, 2, 3], index: 0 };
    let result: Option<i32> = seq_access.next_element().unwrap();
    assert_eq!(result, Some(1));
}

#[test]
fn test_next_element_none() {
    struct TestSeqAccess {
        values: Vec<i32>,
        index: usize,
    }
    
    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = std::convert::Infallible;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index].clone();
                self.index += 1;
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
    }

    let mut seq_access = TestSeqAccess { values: vec![1, 2, 3], index: 3 };
    let result: Option<i32> = seq_access.next_element().unwrap();
    assert_eq!(result, None);
}

