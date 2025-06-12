// Answer 0

#[test]
fn test_visit_enum() {
    struct DummyEnumAccess<'de> {
        _marker: PhantomData<&'de ()>,
    }

    impl<'de> EnumAccess<'de> for DummyEnumAccess<'de> {
        type Error = serde::de::Error;
        type Vars = ();
        
        fn variant<V>(self) -> Result<(Self::Vars, V), Self::Error>
        where
            V: Deserializer<'de>,
        {
            unimplemented!()
        }
    }

    let visitor = DummyEnumAccess { _marker: PhantomData };
    let content_visitor = ContentVisitor { value: PhantomData };

    let result: Result<Content, serde::de::Error> = content_visitor.visit_enum(visitor);
    
    match result {
        Err(err) => assert_eq!(err.to_string(), "untagged and internally tagged enums do not support enum input"),
        _ => panic!("Expected an error return, but got: {:?}", result),
    }
}

