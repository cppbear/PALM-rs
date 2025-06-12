// Answer 0

#[test]
fn test_visit_map_empty_map() {
    struct EmptyMap;
    impl MapAccess<'static> for EmptyMap {
        type Error = std::convert::Infallible;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            Ok(None)
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'static>,
        {
            unreachable!()
        }
    }

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };
    
    let _ = visitor.visit_map(EmptyMap);
}

#[test]
fn test_visit_map_multiple_keys_no_end() {
    struct MultipleKeysMap {
        keys: Vec<Field>,
        index: usize,
    }

    impl MapAccess<'static> for MultipleKeysMap {
        type Error = std::convert::Infallible;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'static>,
        {
            unreachable!()
        }
    }

    let map = MultipleKeysMap {
        keys: vec![Field::End, Field::End],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_end_key_no_value() {
    struct EndKeyNoValueMap {
        keys: Vec<Field>,
        index: usize,
    }

    impl MapAccess<'static> for EndKeyNoValueMap {
        type Error = std::convert::Infallible;

        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index]);
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: Deserialize<'static>,
        {
            unreachable!()
        }
    }

    let map = EndKeyNoValueMap {
        keys: vec![Field::End],
        index: 0,
    };

    let visitor = RangeToVisitor::<i32> {
        expecting: "an integer",
        phantom: std::marker::PhantomData,
    };

    let _ = visitor.visit_map(map);
}

