// Answer 0

#[test]
fn test_deserialize_tuple() {
    use serde::de::{self, Visitor, Deserializer};
    use serde_json::de::Deserializer as JsonDeserializer;

    struct TestVisitor {
        value: Option<(i32, i32)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (i32, i32);

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple of two integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let first: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("expected first element"))?;
            let second: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("expected second element"))?;
            Ok((first, second))
        }
    }

    let json_data = "[1, 2]";
    let deserializer = JsonDeserializer::from_str(json_data);
    let visitor = TestVisitor { value: None };

    let result: Result<(i32, i32), _> = deserializer.deserialize_seq(visitor);
    assert_eq!(result.unwrap(), (1, 2));
}

#[test]
fn test_deserialize_tuple_empty() {
    use serde::de::{self, Visitor, Deserializer};
    use serde_json::de::Deserializer as JsonDeserializer;

    struct TestVisitor {
        value: Option<(i32, i32)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (i32, i32);

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple of two integers")
        }

        fn visit_seq<S>(self, _: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Err(de::Error::custom("expected two elements"))
        }
    }

    let json_data = "[]";
    let deserializer = JsonDeserializer::from_str(json_data);
    let visitor = TestVisitor { value: None };

    let result: Result<(i32, i32), _> = deserializer.deserialize_seq(visitor);
    assert!(result.is_err());
}

