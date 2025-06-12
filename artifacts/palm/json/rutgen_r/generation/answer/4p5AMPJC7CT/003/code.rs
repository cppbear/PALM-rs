// Answer 0

#[test]
fn test_deserialize_struct_with_array() {
    use serde_json::{Value, de::Deserializer, Error};
    use serde::de::{self, Visitor};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<T>(self, seq: T) -> Result<Self::Value, T::Error>
        where
            T: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            let mut access = seq;
            while let Some(value) = access.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let value = Value::Array(vec![
        Value::Number(serde_json::Number::from(1)),
        Value::Number(serde_json::Number::from(2)),
        Value::Number(serde_json::Number::from(3)),
    ]);

    let result: Result<Vec<i32>, Error> = value.deserialize_struct("TestArray", &[], TestVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_struct_with_non_array() {
    use serde_json::{Value, de::Deserializer, Error};
    use serde::de::{self, Visitor};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<T>(self, seq: T) -> Result<Self::Value, T::Error>
        where
            T: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            let mut access = seq;
            while let Some(value) = access.next_element::<i32>()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    let value = Value::Object(serde_json::Map::new());

    let _result: Result<Vec<i32>, Error> = value.deserialize_struct("InvalidStruct", &[], TestVisitor);
}

