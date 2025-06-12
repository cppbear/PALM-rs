// Answer 0

#[test]
fn test_deserialize_seq_with_extra_elements() {
    struct TestVisitor {
        count: usize,
    }
    
    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of zero or more elements")
        }
        
        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            // Push more than 2 elements to cause the error
            seq.next_element::<usize>()?;
            seq.next_element::<usize>()?;
            seq.next_element::<usize>()?; // Extra element
            Ok(())
        }
    }

    let deserializer = PairDeserializer(1u8.into_deserializer(), 2u8.into_deserializer(), PhantomData);
    let visitor = TestVisitor { count: 0 };

    let result: Result<(), _> = deserializer.deserialize_seq(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_seq_with_fewer_elements() {
    struct FewerElementVisitor {
        count: usize,
    }
    
    impl<'de> de::Visitor<'de> for FewerElementVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of zero or more elements")
        }
        
        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            seq.next_element::<usize>()?;
            // Only one element returned
            Ok(())
        }
    }

    let deserializer = PairDeserializer(1u8.into_deserializer(), 2u8.into_deserializer(), PhantomData);
    let visitor = FewerElementVisitor { count: 0 };

    let result: Result<(), _> = deserializer.deserialize_seq(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_seq_with_exact_elements() {
    struct ExactElementVisitor {
        count: usize,
    }
    
    impl<'de> de::Visitor<'de> for ExactElementVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of zero or more elements")
        }
        
        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            seq.next_element::<usize>()?;
            seq.next_element::<usize>()?;
            Ok(())
        }
    }

    let deserializer = PairDeserializer(1u8.into_deserializer(), 2u8.into_deserializer(), PhantomData);
    let visitor = ExactElementVisitor { count: 0 };

    let result: Result<(), _> = deserializer.deserialize_seq(visitor);
    assert!(result.is_ok());
}

