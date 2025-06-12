// Answer 0

#[test]
fn test_deserialize_struct_empty_fields() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn visit_map<M>(self, _: M) -> Result<Self::Value, serde::de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok(())
        }
        
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an empty map")
        }
    }

    let mut data: Vec<Option<(Content, Content)>> = Vec::new();
    let deserializer = FlatMapDeserializer(&mut data);
    let fields: &'static [&'static str] = &[];

    let result: Result<(), _> = deserializer.deserialize_struct("TestStruct", fields, TestVisitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_struct_with_fields() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, serde::de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            while let Some(_) = access.next_entry::<Content, Content>()? {
                self.count += 1;
            }
            Ok(self.count)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with content")
        }
    }

    let content = Content::Bool(true);
    let mut data: Vec<Option<(Content, Content)>> = vec![Some((content.clone(), content))];
    let deserializer = FlatMapDeserializer(&mut data);
    let fields: &'static [&'static str] = &["field1"];

    let visitor = TestVisitor { count: 0 };
    let result: Result<usize, _> = deserializer.deserialize_struct("TestStruct", fields, visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
#[should_panic(expected = "no variant of enum TestStruct found in flattened data")]
fn test_deserialize_struct_no_variants() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, serde::de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Err(serde::de::Error::custom("no variant of enum TestStruct found in flattened data"))
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }
    }

    let mut data: Vec<Option<(Content, Content)>> = vec![];
    let deserializer = FlatMapDeserializer(&mut data);
    let fields: &'static [&'static str] = &["field1"];

    let visitor = TestVisitor;
    deserializer.deserialize_struct("TestStruct", fields, visitor).unwrap();
}

