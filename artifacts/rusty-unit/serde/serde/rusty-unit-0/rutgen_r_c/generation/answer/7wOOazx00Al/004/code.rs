// Answer 0

#[test]
fn test_visit_map_duplicate_end_field() {
    use std::marker::PhantomData;

    struct MockMapAccess {
        keys: Vec<Field>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = MockError;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index];
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<T>(&mut self) -> Result<T, Self::Error>
        where
            T: Deserialize<'de>,
        {
            // Return a mock value for testing
            Ok(42.into()) // Assuming Idx can be an i32
        }
    }

    struct MockVisitor {
        expecting: &'static str,
        phantom: PhantomData<i32>,
    }

    impl<'de> Visitor<'de> for MockVisitor {
        type Value = i32;

        fn expecting(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
        
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut end: Option<i32> = None;
            while let Some(key) = tri!(map.next_key()) {
                match key {
                    Field::End => {
                        if end.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("end"));
                        }
                        end = Some(tri!(map.next_value()));
                    }
                }
            }
            let end = match end {
                Some(end) => end,
                None => return Err(<A::Error as Error>::missing_field("end")),
            };
            Ok(end)
        }
    }

    let mock_map = MockMapAccess {
        keys: vec![Field::End, Field::End],
        index: 0,
    };

    let visitor = MockVisitor {
        expecting: "an end field",
        phantom: PhantomData,
    };

    let result = visitor.visit_map(mock_map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "duplicate field `end`");
}

