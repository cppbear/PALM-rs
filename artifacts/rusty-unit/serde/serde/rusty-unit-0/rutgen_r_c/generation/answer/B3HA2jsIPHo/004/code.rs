// Answer 0

#[test]
fn test_struct_variant_with_none() {
    use std::marker::PhantomData;

    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a struct variant")
        }

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, V::Error> 
        where
            V: de::MapAccess<'de>,
        {
            Ok(())
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error> 
        where
            V: de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    struct MockError;

    impl de::Error for MockError {
        fn custom<T>(_msg: T) -> Self where T: std::fmt::Display { MockError }
    }

    let deserializer = VariantDeserializer::<MockError> {
        value: None,
        err: PhantomData,
    };

    let result: Result<(), MockError> = deserializer.struct_variant(&[], MockVisitor);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(format!("{:?}", e), "invalid type(UnitVariant, \"struct variant\")");
    }
}

