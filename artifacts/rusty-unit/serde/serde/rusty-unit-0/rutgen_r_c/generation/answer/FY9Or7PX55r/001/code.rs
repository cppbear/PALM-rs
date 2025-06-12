// Answer 0

#[test]
fn test_next_element_some_value() {
    struct TestSeqAccess {
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = TestError;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.values.len() {
                let value = &self.values[self.index];
                self.index += 1;
                Ok(Some(value.clone().deserialize(DeserializerImpl)?))
            } else {
                Ok(None)
            }
        }
    }

    struct TestError;

    impl Error for TestError {
        fn custom<T: Display>(msg: T) -> Self {
            println!("{}", msg);
            TestError
        }
    }

    struct DeserializerImpl;

    impl Deserializer<'_> for DeserializerImpl {
        type Error = TestError;
        
        fn deserialize<T>(&self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            // Dummy implementation just for the sake of example
            unimplemented!()
        }
    }

    let mut seq_access = TestSeqAccess {
        values: vec![1, 2, 3],
        index: 0,
    };

    assert_eq!(seq_access.next_element::<i32>().unwrap(), Some(1));
    assert_eq!(seq_access.next_element::<i32>().unwrap(), Some(2));
    assert_eq!(seq_access.next_element::<i32>().unwrap(), Some(3));
    assert_eq!(seq_access.next_element::<i32>().unwrap(), None);
}

#[test]
#[should_panic]
fn test_next_element_out_of_bounds() {
    struct TestSeqAccess {
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> SeqAccess<'de> for TestSeqAccess {
        type Error = TestError;

        fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.index < self.values.len() {
                let value = &self.values[self.index];
                self.index += 1;
                Ok(Some(value.clone().deserialize(DeserializerImpl)?))
            } else {
                Ok(None)
            }
        }
    }

    struct TestError;

    impl Error for TestError {
        fn custom<T: Display>(msg: T) -> Self {
            println!("{}", msg);
            TestError
        }
    }

    struct DeserializerImpl;

    impl Deserializer<'_> for DeserializerImpl {
        type Error = TestError;

        fn deserialize<T>(&self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            // Dummy implementation
            unimplemented!()
        }
    }

    let mut seq_access = TestSeqAccess {
        values: vec![],
        index: 0,
    };

    // This call should panic because there's no more elements to return.
    let _ = seq_access.next_element::<i32>().unwrap();
}

