// Answer 0

#[test]
fn test_visit_array_with_err_from_visitor() {
    use serde::de::{self, Visitor};
    use serde_json::Value;
    
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            Err(de::Error::custom("Custom error from visitor"))
        }
    }

    let array = vec![Value::Null, Value::Bool(true), Value::Number(serde_json::Number::from(42))];
    let visitor = TestVisitor;

    let result = visit_array(array, visitor);
    
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.to_string(), "Custom error from visitor");
    }
}

