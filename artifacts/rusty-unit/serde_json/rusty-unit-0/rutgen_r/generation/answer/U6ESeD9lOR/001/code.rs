// Answer 0

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Option<i32>;

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let value: i32 = deserializer.deserialize_i32(serde::de::value::FromPrimitiveVisitor)?;
            Ok(Some(value))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::value::Error> {
            Ok(None)
        }
    }

    let result: Result<Option<i32>, serde_json::Error> = 
        serde_json::Value::String("10".into()).deserialize::<TestVisitor>();
    
    assert_eq!(result.unwrap(), Some(10));
}

#[test]
#[should_panic]
fn test_deserialize_option_none() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Option<i32>;

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            panic!("This should not be called for None");
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::value::Error> {
            Ok(None)
        }
    }

    let result: Result<Option<i32>, serde_json::Error> = 
        serde_json::Value::Null.deserialize::<TestVisitor>();
    
    assert_eq!(result.unwrap(), None);
}

