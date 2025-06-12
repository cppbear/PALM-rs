// Answer 0

#[test]
fn test_deserialize_tuple_struct_with_seq() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            
            while let Some(value) = seq.next_element::<Value>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let value = Value::Array(vec![
        Value::Number(Number { n: 1 }), 
        Value::String(String::from("test"))
    ]);

    let result: Result<Vec<Value>, Error> = value.deserialize_tuple_struct("TestStruct", 2, TestVisitor);
    
    assert!(result.is_ok());
    let deserialized = result.unwrap();
    assert_eq!(deserialized.len(), 2);
    assert_eq!(deserialized[0], Value::Number(Number { n: 1 }));
    assert_eq!(deserialized[1], Value::String(String::from("test")));
}

#[test]
fn test_deserialize_tuple_struct_empty() {
    struct EmptyVisitor;

    impl<'de> Visitor<'de> for EmptyVisitor {
        type Value = Vec<Value>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(Vec::new())
        }
    }

    let value = Value::Array(vec![]);

    let result: Result<Vec<Value>, Error> = value.deserialize_tuple_struct("EmptyStruct", 0, EmptyVisitor);
    
    assert!(result.is_ok());
    let deserialized = result.unwrap();
    assert_eq!(deserialized.len(), 0);
}

