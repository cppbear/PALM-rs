// Answer 0

#[test]
fn test_visit_enum() {
    use serde::de::{self, EnumAccess, Visitor};
    use std::marker::PhantomData;
    
    struct TestVisitor {
        value: String,
    }

    impl<'de> EnumAccess<'de> for TestVisitor {
        type Error = de::Error;
        type Variant = TestVariant;

        fn variant<E>(self, _name: &'static str) -> Result<(Self::Variant, E), Self::Error>
        where
            E: Visitor<'de>,
        {
            Ok((TestVariant { value: self.value }, de::Error::custom("not implemented")))
        }
    }

    struct TestVariant {
        value: String,
    }

    struct ContentVisitor;

    impl ContentVisitor {
        fn new() -> Self {
            ContentVisitor
        }

        fn visit_enum<V>(self, _visitor: V) -> Result<TagOrContent<Self>, V::Error>
        where
            V: EnumAccess<'de>,
        {
            Err(de::Error::custom("visit_enum not implemented"))
        }
    }

    enum TagOrContent<V> {
        Content(V),
    }

    let visitor = TestVisitor { value: String::from("test") };
    let result: Result<TagOrContent<TestVariant>, _> = ContentVisitor::new().visit_enum(visitor);
    
    assert!(result.is_err());
}

