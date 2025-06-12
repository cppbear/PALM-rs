// Answer 0

#[test]
fn test_visit_map_with_duplicate_start_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

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
            V: Deserialize<'de>,
        {
            if self.values.is_empty() {
                Err(serde::de::value::Error::custom("no values available"))
            } else {
                let value = self.values.remove(0);
                Ok(value as V)
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![Field::Start, Field::Start],
        values: vec![0, 1],
        index: 0,
    };

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData::<i32>,
    };

    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_with_missing_start_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

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
            V: Deserialize<'de>,
        {
            // Only providing value for `end`
            if self.index == 0 {
                let value = self.values.remove(0);
                Ok(value as V)
            } else {
                Err(serde::de::value::Error::custom("no value available"))
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![Field::End],
        values: vec![1],
        index: 0,
    };

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData::<i32>,
    };

    let _ = visitor.visit_map(map_access);
} 

#[test]
fn test_visit_map_with_missing_end_field() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

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
            V: Deserialize<'de>,
        {
            if self.values.is_empty() {
                Err(serde::de::value::Error::custom("no values available"))
            } else {
                let value = self.values.remove(0);
                Ok(value as V)
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![Field::Start],
        values: vec![0],
        index: 0,
    };

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData::<i32>,
    };

    let _ = visitor.visit_map(map_access);
} 

#[test]
fn test_visit_map_with_valid_input() {
    struct MockMapAccess {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = serde::de::value::Error;

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
            V: Deserialize<'de>,
        {
            if self.index == 2 {
                // Only providing one valid value per key
                return Ok(self.values.remove(0) as V);
            }
            Err(serde::de::value::Error::custom("no value available"))
        }
    }

    let map_access = MockMapAccess {
        keys: vec![Field::Start, Field::End],
        values: vec![0, 1],
        index: 0,
    };

    let visitor = RangeVisitor {
        expecting: "a range",
        phantom: std::marker::PhantomData::<i32>,
    };

    let _ = visitor.visit_map(map_access);
}

