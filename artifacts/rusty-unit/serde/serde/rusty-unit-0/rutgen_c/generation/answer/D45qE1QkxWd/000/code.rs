// Answer 0

#[test]
fn test_deserialize_u16_valid() {
    struct TestVisitor {
        value: Option<u16>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u16;

        fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
            Ok(value)
        }
        
        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    let content = Content::U16(42);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    
    let result: Result<u16, _> = deserializer.deserialize_u16(TestVisitor { value: None });
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_u16_invalid_type() {
    struct TestVisitor {
        value: Option<u16>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = u16;

        fn visit_u16<E>(self, _: u16) -> Result<Self::Value, E> {
            unimplemented!()
        }

        fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
            unimplemented!()
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };

    let result: Result<u16, _> = deserializer.deserialize_u16(TestVisitor { value: None });
    assert!(result.is_err());
}

