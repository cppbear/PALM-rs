// Answer 0

#[test]
fn test_visit_map_duplicate_end_field() {
    use std::collections::HashMap;
    use serde::de::{self, MapAccess};

    struct DummyMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl DummyMap {
        fn new() -> Self {
            DummyMap {
                keys: vec![Field::Start, Field::End, Field::End],
                values: vec![1, 2, 3],
                index: 0,
            }
        }
    }

    impl<'de> MapAccess<'de> for DummyMap {
        type Error = de::value::Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                // Use the appropriate value
                let value: i32 = self.values[self.index - 1];
                self.index += 1;
                Ok(value as V)
            } else {
                Err(de::value::Error::custom("no more values"))
            }
        }
    }

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (i32, i32);

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with 'start' and 'end' fields")
        }
    }

    let visitor = TestVisitor;
    let mut map = DummyMap::new();
    let result: Result<(i32, i32), _> = visitor.visit_map(&mut map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "duplicate field `end`");
}

