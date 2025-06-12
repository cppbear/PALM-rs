// Answer 0

#[test]
fn test_deserialize_option_ok() {
    use crate::de::Visitor;

    struct TestVisitor {
        value: Option<Content<'static>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'static>;

        fn __private_visit_untagged_option<V>(self, _: FlatMapDeserializer) -> Result<Self::Value, ()> {
            Ok(self.value.unwrap())
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(Content::Unit)
        }

        fn visit_none(self) -> Result<Self::Value, Self::Error> {
            Ok(Content::None)
        }

        fn visit_some(self, value: Self::Value) -> Result<Self::Value, Self::Error> {
            Ok(Content::Some(Box::new(value)))
        }
    }

    let content = Content::U8(5);
    let mut vec = vec![Some((content.clone(), Content::None))];
    let deserializer = FlatMapDeserializer(&mut vec, std::marker::PhantomData);

    let visitor = TestVisitor { value: Some(content) };
    let result = deserializer.deserialize_option(visitor);
    
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_deserialize_option_fail() {
    use crate::de::Visitor;

    struct FailingVisitor;

    impl<'de> Visitor<'de> for FailingVisitor {
        type Value = Content<'static>;

        fn __private_visit_untagged_option<V>(self, _: FlatMapDeserializer) -> Result<Self::Value, ()> {
            Err(())
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(Content::Unit)
        }
    }

    let mut vec = vec![None];
    let deserializer = FlatMapDeserializer(&mut vec, std::marker::PhantomData);

    let visitor = FailingVisitor {};
    let _result = deserializer.deserialize_option(visitor);
}

