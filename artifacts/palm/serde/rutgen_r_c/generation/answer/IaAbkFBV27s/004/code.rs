// Answer 0

#[test]
fn test_visit_map_missing_start_field() {
    use crate::de::{MapAccess, Error};

    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        current_index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Error;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.current_index < self.keys.len() {
                let key = self.keys[self.current_index];
                self.current_index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'de>,
        {
            let value = self.values.remove(0);
            Ok(value as V)
        }
    }

    let mock_map = MockMapAccess {
        keys: vec![Field::End],
        values: vec![10],
        current_index: 0,
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32",
        phantom: std::marker::PhantomData,
    };

    let result: Result<(i32, i32), _> = visitor.visit_map(mock_map);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, Error::missing_field("start"));
    }
}

