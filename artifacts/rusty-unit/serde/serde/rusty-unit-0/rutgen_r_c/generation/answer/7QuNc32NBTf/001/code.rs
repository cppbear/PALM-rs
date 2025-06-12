// Answer 0

#[test]
fn test_tuple_variant_error() {
    struct DummyVisitor;

    impl<'de> de::Visitor<'de> for DummyVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("dummy visitor")
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(())
        }
    }

    struct DummyError;

    impl de::Error for DummyError {
        fn custom<T>(msg: T) -> Self
        where
            T: std::fmt::Display,
        {
            DummyError
        }
    }

    let unit_only = UnitOnly::<DummyError> {
        marker: std::marker::PhantomData,
    };
    
    let result = unit_only.tuple_variant(0usize, DummyVisitor);
    
    assert!(result.is_err());
    if let Err(ref e) = result {
        assert_eq!(
            e.to_string(),
            "invalid type: unit variant, expected tuple variant"
        );
    }
}

