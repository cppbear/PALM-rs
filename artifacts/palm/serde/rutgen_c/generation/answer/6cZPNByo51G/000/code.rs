// Answer 0

#[test]
fn test_deserialize_option_some() {
    struct VisitorMock {
        called: bool,
    };

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = Option<Content<'de>>;

        fn __private_visit_untagged_option<V>(self, deserializer: FlatMapDeserializer) -> Result<Self::Value, ()> {
            self.called = true;
            Ok(Some(Content::Bool(true)))
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }
    }

    let mut data = vec![Some((Content::Str("key"), Content::Bool(true)))];
    let deserializer = FlatMapDeserializer(&mut data, std::marker::PhantomData);
    let visitor = VisitorMock { called: false };

    let result = deserializer.deserialize_option(visitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(Content::Bool(true)));
    assert!(visitor.called);
}

#[test]
fn test_deserialize_option_none() {
    struct VisitorMock {
        called: bool,
    };

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = Option<Content<'de>>;

        fn __private_visit_untagged_option<V>(
            self,
            _deserializer: FlatMapDeserializer,
        ) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }
    }

    let mut data: Vec<Option<(Content, Content)>> = vec![None];
    let deserializer = FlatMapDeserializer(&mut data, std::marker::PhantomData);
    let visitor = VisitorMock { called: false };

    let result = deserializer.deserialize_option(visitor);

    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_deserialize_option_panic() {
    struct VisitorMock {}

    impl<'de> Visitor<'de> for VisitorMock {
        type Value = Option<Content<'de>>;

        fn __private_visit_untagged_option<V>(
            self,
            _deserializer: FlatMapDeserializer,
        ) -> Result<Self::Value, ()> {
            panic!("Panic in visit_untagged_option");
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }
    }

    let mut data = vec![Some((Content::Str("key"), Content::Bool(true)))];
    let deserializer = FlatMapDeserializer(&mut data, std::marker::PhantomData);
    let visitor = VisitorMock {};

    deserializer.deserialize_option(visitor);
}

