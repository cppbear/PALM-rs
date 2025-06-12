// Answer 0

#[test]
fn test_deserialize_enum_multiple_keys() {
    struct MockVisitor {
        value: Option<String>,
        error: Option<Error>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<(String, Option<&'de Value>)>;

        fn visit_enum<V>(self, _data: V) -> Result<Self::Value, Error>
        where
            V: EnumAccess<'de>,
        {
            Ok(self.value.clone().map(|v| (v, None)))
        }
    }

    let value = Value::Object(Map {
        map: vec![
            (String::from("key1"), Value::String(String::from("variant1"))),
            (String::from("key2"), Value::String(String::from("variant2"))),
        ].into_iter().collect(),
    });

    let visitor = MockVisitor { value: Some(String::from("variant1")), error: None };
    let result: Result<Option<(String, Option<&Value>)>, Error> = value.deserialize_enum("TestEnum", &["variant1", "variant2"], visitor);

    assert_eq!(result, Err(serde::de::Error::invalid_value(
        Unexpected::Map,
        &"map with a single key"
    )));
}

#[test]
fn test_deserialize_enum_empty() {
    struct MockVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = Option<(String, Option<&'de Value>)>;

        fn visit_enum<V>(self, _data: V) -> Result<Self::Value, Error>
        where
            V: EnumAccess<'de>,
        {
            Ok(self.value.clone().map(|v| (v, None)))
        }
    }

    let value = Value::Object(Map {
        map: std::collections::BTreeMap::new(),
    });

    let visitor = MockVisitor { value: Some(String::from("variant1")) };
    let result: Result<Option<(String, Option<&Value>)>, Error> = value.deserialize_enum("EmptyEnum", &["variant1"], visitor);

    assert_eq!(result, Err(serde::de::Error::invalid_value(
        Unexpected::Map,
        &"map with a single key"
    )));
}

