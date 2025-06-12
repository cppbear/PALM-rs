// Answer 0

#[test]
fn test_deserialize_bool_success() {
    struct VisitorMock {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = bool;

        fn visit_bool(self, value: bool) -> Result<Self::Value, ()> {
            self.value = Some(value);
            Ok(value)
        }

        fn visit_none(self) -> Result<Self::Value, ()> {
            self.value = None;
            Ok(false)
        }

        fn invalid_type(self, _: &dyn Expected) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };
    
    let mut visitor = VisitorMock { value: None };
    let result = deserializer.deserialize_bool(visitor);
    
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_deserialize_bool_failure() {
    struct VisitorMock {
        value: Option<bool>,
    }

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = bool;

        fn visit_bool(self, _: bool) -> Result<Self::Value, ()> {
            Err(()) // we should not reach here
        }

        fn visit_none(self) -> Result<Self::Value, ()> {
            Err(()) // we should not reach here
        }

        fn invalid_type(self, _: &dyn Expected) -> Result<Self::Value, ()> {
            Err(())
        }
    }

    let content = Content::String("not_a_bool".to_string());
    let deserializer = ContentDeserializer {
        content,
        err: std::marker::PhantomData,
    };

    let visitor = VisitorMock { value: None };
    let result = deserializer.deserialize_bool(visitor);
    
    assert!(result.is_err());
}

