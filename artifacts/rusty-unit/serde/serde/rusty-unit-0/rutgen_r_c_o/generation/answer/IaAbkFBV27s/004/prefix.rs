// Answer 0

#[test]
fn test_visit_map_missing_start() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl MapAccess<'static> for TestMap {
        type Error = serde::de::value::Error;
        
        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1].clone();
                self.index += 1;
                Ok(value as T) // Assuming it will work for i32
            } else {
                Err(serde::de::value::Error::custom("out of values"))
            }
        }
    }

    let keys = vec![Field::End];
    let values = vec![10];

    let result = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    }
    .visit_map(TestMap { keys, values, index: 0 });

    // The returned result should be an Err indicating the missing "start"
}

#[test]
fn test_visit_map_duplicate_start() {
    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl MapAccess<'static> for TestMap {
        type Error = serde::de::value::Error;
        
        fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
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
            let value = self.values[self.index - 1].clone();
            self.index += 1;
            Ok(value as T) // Assuming it will work for i32
        }
    }

    let keys = vec![Field::Start, Field::Start];
    let values = vec![5, 10];

    let result = RangeVisitor::<i32> {
        expecting: "a range",
        phantom: std::marker::PhantomData,
    }
    .visit_map(TestMap { keys, values, index: 0 });

    // The returned result should indicate a duplicate_field error
}

