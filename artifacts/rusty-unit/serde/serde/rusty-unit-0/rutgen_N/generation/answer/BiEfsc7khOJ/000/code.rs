// Answer 0

#[test]
fn test_visit_content_map() {
    use serde::de::{self, Visitor, MapAccess};
    use std::marker::PhantomData;

    struct TestVisitor {
        value: Vec<(Content<'static>, Content<'static>)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<(Content<'de>, Content<'de>)>;
        
        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(self.value)
        }
    }

    struct MapDeserializer<I> {
        iter: I,
    }

    impl<I> MapDeserializer<I> {
        fn new(iter: I) -> Self {
            MapDeserializer { iter }
        }

        fn end(self) -> Result<(), de::Error> {
            Ok(())
        }
    }

    struct Content<'a> {
        data: &'a str,
    }

    let content = vec![
        (Content { data: "key1" }, Content { data: "value1" }),
        (Content { data: "key2" }, Content { data: "value2" }),
    ];

    let visitor = TestVisitor { value: vec![] };
    
    let result: Result<Vec<(Content<'static>, Content<'static>)>, de::Error> =
        visit_content_map(content, visitor);
    
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.len(), 2);
}

