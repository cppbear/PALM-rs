// Answer 0

#[test]
fn test_deserialize_f64_success() {
    struct TestVisitor {
        value: Option<f64>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<f64>;

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Some(v))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    let content = Content::F64(3.14);
    let deserializer = ContentDeserializer::<value::Error> {
        content,
        err: PhantomData,
    };

    let result = deserializer.deserialize_f64(TestVisitor { value: None }).unwrap();
    assert_eq!(result, Some(3.14));
}

#[test]
fn test_deserialize_f64_invalid_type() {
    struct TestVisitor {
        value: Option<f64>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<f64>;

        fn visit_f64<E>(self, _v: f64) -> Result<Self::Value, E> {
            unreachable!()
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer::<value::Error> {
        content,
        err: PhantomData,
    };

    let result = deserializer.deserialize_f64(TestVisitor { value: None });
    assert!(result.is_err());
}

