// Answer 0

#[test]
fn test_deserialize_enum_with_string_content() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<String>;

        fn visit_enum(self, _data: EnumRefDeserializer<'de, String, PhantomData<String>>) -> Result<Self::Value, serde::de::Error> {
            Ok(self.value)
        }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn new_string(content: &str) -> Self {
            TestContent {
                content: Content::String(content.to_string()),
            }
        }

        fn deserialize_enum<V>(&self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'de>,
        {
            super::deserialize_enum(&self.content, _name, _variants, visitor)
        }
    }

    let content = TestContent::new_string("variant_value");
    let visitor = TestVisitor { value: Some("variant_value".to_string()) };
    let result = content.deserialize_enum("TestEnum", &["variant1", "variant2"], visitor);
    assert_eq!(result.ok(), Some(Some("variant_value".to_string())));
}

#[test]
fn test_deserialize_enum_with_map_content() {
    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<String>;

        fn visit_enum(self, _data: EnumRefDeserializer<'de, &'de str, PhantomData<&'de str>>) -> Result<Self::Value, serde::de::Error> {
            Ok(self.value)
        }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn new_map(content: std::collections::HashMap<&'static str, &'static str>) -> Self {
            TestContent {
                content: Content::Map(content.into_iter().collect()),
            }
        }

        fn deserialize_enum<V>(&self, _name: &str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'de>,
        {
            super::deserialize_enum(&self.content, _name, _variants, visitor)
        }
    }

    let mut map = std::collections::HashMap::new();
    map.insert("variant1", "value1");
    let content = TestContent::new_map(map);
    let visitor = TestVisitor { value: Some("value1".to_string()) };
    let result = content.deserialize_enum("TestEnum", &["variant1", "variant2"], visitor);
    assert_eq!(result.ok(), Some(Some("value1".to_string())));
}

