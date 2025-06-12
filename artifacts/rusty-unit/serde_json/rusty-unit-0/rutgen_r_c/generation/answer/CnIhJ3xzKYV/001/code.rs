// Answer 0

#[test]
fn test_deserialize_any_empty_map() {
    use serde::de::visitor::Visitor;
    
    struct TestVisitor {
        return_err: bool,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, serde::de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            if self.return_err {
                Err(serde::de::Error::custom("Visit map error"))
            } else {
                Ok(())
            }
        }
    }

    let mut map = serde_json::Map::new();
    let value = serde_json::Value::Object(map);

    let visitor = TestVisitor { return_err: true };
    let result: Result<(), serde_json::Error> = value.deserialize_any(visitor);

    assert!(result.is_err());
}

#[test]
fn test_deserialize_any_non_empty_map() {
    use serde::de::visitor::Visitor;
    
    struct NonEmptyTestVisitor {
        return_err: bool,
    }

    impl<'de> Visitor<'de> for NonEmptyTestVisitor {
        type Value = ();

        fn visit_map<M>(self, _: M) -> Result<Self::Value, serde::de::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            if self.return_err {
                Err(serde::de::Error::custom("Visit map error"))
            } else {
                Ok(())
            }
        }
    }

    let mut map = serde_json::Map::new();
    map.insert("key".to_string(), serde_json::Value::Null);
    let value = serde_json::Value::Object(map);

    let visitor = NonEmptyTestVisitor { return_err: true };
    let result: Result<(), serde_json::Error> = value.deserialize_any(visitor);

    assert!(result.is_err());
}

