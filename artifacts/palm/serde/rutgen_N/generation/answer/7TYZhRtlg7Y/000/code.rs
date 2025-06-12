// Answer 0

#[test]
fn test_deserialize_struct_with_sequence() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some(value) = seq.next_element()? {
                result.push(value);
            }
            Ok(result)
        }
    }

    let content = Content::Seq(vec![1.into(), 2.into(), 3.into()]);
    let deserializer = Deserializer::new(content);
    let result: Result<Vec<i32>, _> = deserializer.deserialize_struct("test", &[], TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_struct_with_map() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = std::collections::HashMap<String, i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of strings to integers")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            let mut result = std::collections::HashMap::new();
            while let Some((key, value)) = map.next_entry()? {
                result.insert(key, value);
            }
            Ok(result)
        }
    }

    let content = Content::Map(vec![(String::from("one"), 1.into()), (String::from("two"), 2.into())]);
    let deserializer = Deserializer::new(content);
    let result: Result<std::collections::HashMap<String, i32>, _> = deserializer.deserialize_struct("test", &[], TestVisitor);
    let mut expected = std::collections::HashMap::new();
    expected.insert(String::from("one"), 1);
    expected.insert(String::from("two"), 2);
    assert_eq!(result.unwrap(), expected);
}

#[test]
#[should_panic]
fn test_deserialize_struct_with_invalid_type() {
    let content = Content::Other;
    let deserializer = Deserializer::new(content);
    let _result: Result<(), _> = deserializer.deserialize_struct("test", &[], TestVisitor);
}

