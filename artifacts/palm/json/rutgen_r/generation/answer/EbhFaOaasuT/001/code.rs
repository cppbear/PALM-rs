// Answer 0

#[test]
fn test_next_key_seed_with_some() {
    use serde::de::{DeserializeSeed, Deserializer};
    use std::borrow::Cow;
    use std::collections::HashMap;

    struct TestDeserializer;

    impl<'de> DeserializeSeed<'de> for TestDeserializer {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let key: Cow<str> = deserializer.deserialize_str(KeyVisitor)?;
            Ok(key.into_owned())
        }
    }

    struct MapKeyDeserializer {
        key: Cow<str>,
    }

    struct KeyVisitor;

    impl serde::de::Visitor<'_> for KeyVisitor {
        type Value = Cow<str>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
            Ok(Cow::Owned(value.to_string()))
        }
    }

    struct IterTest {
        iter: std::iter::Once<(String, String)>,
        value: Option<String>,
    }

    impl IterTest {
        fn new() -> Self {
            let data = HashMap::from([("key".to_string(), "value".to_string())]);
            IterTest {
                iter: data.into_iter().next(),
                value: None,
            }
        }

        fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, serde_json::Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.iter.next() {
                Some((key, value)) => {
                    self.value = Some(value);
                    let key_de = MapKeyDeserializer {
                        key: Cow::Owned(key),
                    };
                    seed.deserialize(key_de).map(Some)
                }
                None => Ok(None),
            }
        }
    }

    let mut deserializer = IterTest::new();
    let result = deserializer.next_key_seed(TestDeserializer).unwrap();

    assert_eq!(result, Some("key".to_string()));
}

