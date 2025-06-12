// Answer 0

#[test]
fn test_visit_array_err_non_sequence_value() {
    let array = vec![
        Value::Bool(true),
        Value::String("test".to_owned()),
        Value::Number(Number::from(123)),
    ];
    let visitor = MockVisitor;
    let _ = visit_array(array, visitor);
}

#[test]
fn test_visit_array_err_malformed_structure() {
    let array = vec![
        Value::Object(Map::new()),
        Value::Array(vec![Value::Null]),
        Value::String("malformed".to_owned()),
    ];
    let visitor = MockVisitor;
    let _ = visit_array(array, visitor);
}

#[test]
fn test_visit_array_err_empty_elements() {
    let array = vec![Value::Null, Value::Null];
    let visitor = MockVisitor;
    let _ = visit_array(array, visitor);
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = ();
    
    fn visit_seq<S>(self, _: S) -> Result<Self::Value, Error>
    where
        S: SeqAccess<'de>,
    {
        Err(Error::syntax(ErrorCode::ExpectedNumericKey, 0, 0))
    }
    
    forward_to_deserialize_any! {
        bool str string char bytes byte_buf option unit
        array map enum tuple newtype_struct identifier
    }
}

