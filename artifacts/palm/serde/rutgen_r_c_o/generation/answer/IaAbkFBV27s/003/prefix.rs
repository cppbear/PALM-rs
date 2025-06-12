// Answer 0

#[test]
fn test_visit_map_missing_end() {
    struct DummyMap {
        keys: Vec<Field>,
        index: usize,
    }

    impl MapAccess<'static> for DummyMap {
        type Error = ();
        
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
            T: Deserialize<'static>,
        {
            // We can simulate some valid values, just return 
            // a placeholder since we're testing for an error here.
            Ok(Default::default())
        }
    }

    let mut map = DummyMap { 
        keys: vec![Field::Start], 
        index: 0 
    };

    let visitor = RangeVisitor::<i32> {
        expecting: "an i32 range",
        phantom: PhantomData,
    };

    let _ = visitor.visit_map(&mut map);
}

