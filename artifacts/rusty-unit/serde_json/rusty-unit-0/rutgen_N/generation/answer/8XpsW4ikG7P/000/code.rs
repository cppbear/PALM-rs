// Answer 0

#[test]
fn test_deserialize_seq_array() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let json_value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]);
    let result: Result<Vec<i32>, Error> = json_value.deserialize_seq(TestVisitor);
    assert_eq!(result, Ok(vec![1, 2]));
}

#[test]
fn test_deserialize_seq_not_array() {
    use serde::de::{self, Visitor};
    use serde_json::{Value, Error};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array")
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }
    }

    let json_value = Value::Number(3.into());
    let result: Result<(), Error> = json_value.deserialize_seq(TestVisitor);
    assert!(result.is_err());
}

