// Answer 0

#[test]
fn test_next_value_with_valid_deserialization() {
    use serde::de::{Deserializer, MapAccess, SeqAccess};
    use serde::Deserialize;

    struct TestMapAccess {
        data: Vec<(String, String)>,
        index: usize,
    }

    impl MapAccess<'_> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(
            &mut self,
            _seed: K,
        ) -> Result<Option<String>, Self::Error>
        where
            K: Deserialize<'_>,
        {
            if self.index < self.data.len() {
                let key = self.data[self.index].0.clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _: V) -> Result<V, Self::Error>
        where
            V: Deserialize<'_>,
        {
            if self.index == 0 || self.index > self.data.len() {
                panic!("next_value called without next_key or out of bounds");
            }
            let value = self.data[self.index - 1].1.clone();
            serde_json::from_str(&value).map_err(|_| serde::de::value::Error::custom("failed to deserialize"))
        }
    }

    let data = vec![
        ("key1".to_string(), r#""value1""#.to_string()),
        ("key2".to_string(), r#""value2""#.to_string()),
    ];

    let mut map_access = TestMapAccess { data, index: 0 };

    // Test valid return values
    let value1: String = map_access.next_value().unwrap();
    assert_eq!(value1, "value1");

    let value2: String = map_access.next_value().unwrap();
    assert_eq!(value2, "value2");
}

#[test]
#[should_panic(expected = "next_value called without next_key or out of bounds")]
fn test_next_value_panics_before_next_key() {
    use serde::de::{Deserializer, MapAccess};

    struct TestMapAccess {
        data: Vec<(String, String)>,
        index: usize,
    }

    impl MapAccess<'_> for TestMapAccess {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(
            &mut self,
            _: K,
        ) -> Result<Option<String>, Self::Error>
        where
            K: Deserialize<'_>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _: V) -> Result<V, Self::Error>
        where
            V: Deserialize<'_>,
        {
            if self.index == 0 {
                panic!("next_value called without next_key");
            }
            Err(serde::de::value::Error::custom("failed to deserialize"))
        }
    }

    let data: Vec<(String, String)> = vec![];

    let mut map_access = TestMapAccess { data, index: 0 };

    // This should panic since next_value is called without calling next_key first
    let _: String = map_access.next_value().unwrap();
}

