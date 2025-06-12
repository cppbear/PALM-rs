// Answer 0

#[test]
fn test_deserialize_tuple_struct_valid_input() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let input_data = r#" [1, 2, 3] "#;
    let deserializer = serde_json::Deserializer::from_str(input_data);
    
    let result: Result<Vec<i32>, serde_json::Error> = 
        deserializer.deserialize_tuple_struct("TupleStruct", 3, TestVisitor);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_tuple_struct_invalid_length() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let len = seq.size_hint().0;
            Err(serde::de::Error::invalid_length(len, &self))
        }
    }

    let input_data = r#" [1, 2] "#; // Length is less than expected
    let deserializer = serde_json::Deserializer::from_str(input_data);
    
    let result: Result<Vec<i32>, serde_json::Error> = 
        deserializer.deserialize_tuple_struct("TupleStruct", 3, TestVisitor);

    assert!(result.is_err());
}

#[test]
fn test_deserialize_tuple_struct_panic_on_incorrect_type() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of i32")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, serde::de::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let input_data = r#" ["1", "2", "3"] "#; // invalid type, expects i32
    let deserializer = serde_json::Deserializer::from_str(input_data);
    
    let result: Result<Vec<i32>, serde_json::Error> = 
        deserializer.deserialize_tuple_struct("TupleStruct", 3, TestVisitor);

    assert!(result.is_err());
}

