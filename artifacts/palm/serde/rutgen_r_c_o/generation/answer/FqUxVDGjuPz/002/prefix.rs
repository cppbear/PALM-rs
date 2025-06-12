// Answer 0

#[test]
fn test_newtype_variant_seed_none() {
    struct TestError;
    
    impl de::Error for TestError {
        fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
            TestError
        }
    }

    struct TestSeed;

    impl<'de> de::DeserializeSeed<'de> for TestSeed {
        type Value = Result<(), TestError>;
        
        fn deserialize<Deserializer>(self, deserializer: Deserializer) -> Self::Value
        where
            Deserializer: de::Deserializer<'de>,
        {
            Ok(())
        }
    }

    let variant_deserializer: VariantDeserializer<TestError> = VariantDeserializer {
        value: None,
        err: PhantomData,
    };
    
    let result: Result<(), TestError> = variant_deserializer.newtype_variant_seed(TestSeed);
}

