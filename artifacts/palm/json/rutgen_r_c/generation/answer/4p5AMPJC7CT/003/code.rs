// Answer 0

#[test]
fn test_deserialize_struct_with_array() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::Deserialize;

    struct TestVisitor {
        result: Vec<Value>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let json_array = Value::Array(vec![
        Value::String(String::from("first")),
        Value::String(String::from("second")),
    ]);

    let result = json_array.deserialize_struct("TestStruct", &[] , TestVisitor{ result: Vec::new() }).unwrap();
    
    assert_eq!(result, vec![
        Value::String(String::from("first")),
        Value::String(String::from("second")),
    ]);
}

#[test]
#[should_panic]
fn test_deserialize_struct_with_non_array() {
    use serde_json::Value;
    use serde::de::Visitor;
    use serde::Deserialize;

    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_seq<A>(self, _seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    let json_object = Value::Object(serde_json::Map::new());

    json_object.deserialize_struct("TestStruct", &[], PanicVisitor).unwrap();
}

