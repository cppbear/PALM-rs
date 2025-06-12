// Answer 0

#[test]
fn test_newtype_variant_seed_with_some_value() {
    use serde::de::{DeserializeSeed, Visitor};
    use core::marker::PhantomData;

    struct TestSeed {
        value: i32,
    }

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = i32;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let res: i32 = Deserialize::deserialize(deserializer)?;
            Ok(res)
        }
    }

    let value = Some(Value::Number(Number::from(42)));
    let deserializer = VariantRefDeserializer { value };

    let result: Result<i32, Error> = deserializer.newtype_variant_seed(TestSeed { value: 0 });

    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_newtype_variant_seed_with_none_value() {
    use serde::de::{DeserializeSeed, Visitor};
    use core::marker::PhantomData;

    struct TestSeed;

    impl<'de> DeserializeSeed<'de> for TestSeed {
        type Value = ();

        fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(())
        }
    }

    let deserializer = VariantRefDeserializer { value: None };

    let result: Result<(), Error> = deserializer.newtype_variant_seed(TestSeed);

    assert!(result.is_err());
}

