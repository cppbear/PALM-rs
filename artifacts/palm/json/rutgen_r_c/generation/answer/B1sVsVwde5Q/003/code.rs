// Answer 0

#[test]
fn test_deserialize_struct_array_success() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(vec![Value::Bool(true), Value::Number(Number { n: 42.into() })])
        }
    }

    let value = Value::Array(vec![Value::Bool(true), Value::Number(Number { n: 42.into() })]);
    let visitor = TestVisitor;

    let result = value.deserialize_struct("test", &["field1", "field2"], visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![Value::Bool(true), Value::Number(Number { n: 42.into() })]);
}

#[test]
fn test_deserialize_struct_object() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_map<V>(self, _map: &mut V) -> Result<Self::Value, Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            Ok("Visited Map".to_owned())
        }
    }

    let value = Value::Object(Map { map: Default::default() });
    let visitor = TestVisitor;

    let result = value.deserialize_struct("test", &["field1", "field2"], visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Visited Map");
}

#[test]
#[should_panic]
fn test_deserialize_struct_invalid_type() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Error> {
            Ok(())
        }
    }

    let value = Value::Null;
    let visitor = TestVisitor;

    let _result = value.deserialize_struct("test", &["field1"], visitor);
}

#[test]
fn test_deserialize_struct_array_empty() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    let value = Value::Array(vec![]);
    let visitor = TestVisitor;

    let result = value.deserialize_struct("test", &["field1", "field2"], visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![]);
}

