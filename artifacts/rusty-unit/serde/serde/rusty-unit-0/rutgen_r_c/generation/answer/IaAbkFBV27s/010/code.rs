// Answer 0

#[test]
fn test_visit_map_valid_input() {
    use std::marker::PhantomData;
    use std::collections::HashMap;

    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl TestMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            Self { keys, values, index: 0 }
        }
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::Error;

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
            if self.index - 1 < self.values.len() {
                let value: T = self.values[self.index - 1].try_into().map_err(|_| serde::de::Error::custom("Conversion error"))?;
                Ok(value)
            } else {
                Err(serde::de::Error::custom("No value for key"))
            }
        }
    }

    struct TestVisitor {
        phantom: PhantomData<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (i32, i32);
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut start: Option<i32> = None;
            let mut end: Option<i32> = None;
            while let Some(key) = tri!(map.next_key()) {
                match key {
                    Field::Start => {
                        if start.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("start"));
                        }
                        start = Some(tri!(map.next_value()));
                    }
                    Field::End => {
                        if end.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("end"));
                        }
                        end = Some(tri!(map.next_value()));
                    }
                }
            }
            let start = start.ok_or_else(|| <A::Error as Error>::missing_field("start"))?;
            let end = end.ok_or_else(|| <A::Error as Error>::missing_field("end"))?;
            Ok((start, end))
        }
    }

    let keys = vec![Field::Start, Field::End];
    let values = vec![1, 2];
    let test_map = TestMap::new(keys, values);
    let visitor = TestVisitor { phantom: PhantomData };
    let result = visitor.visit_map(test_map);
    assert_eq!(result, Ok((1, 2)));
}

#[test]
#[should_panic(expected = "missing field `start`")]
fn test_visit_map_missing_start() {
    use std::marker::PhantomData;
    use std::collections::HashMap;

    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl TestMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            Self { keys, values, index: 0 }
        }
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::Error;

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
            if self.index - 1 < self.values.len() {
                let value: T = self.values[self.index - 1].try_into().map_err(|_| serde::de::Error::custom("Conversion error"))?;
                Ok(value)
            } else {
                Err(serde::de::Error::custom("No value for key"))
            }
        }
    }

    struct TestVisitor {
        phantom: PhantomData<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (i32, i32);
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut start: Option<i32> = None;
            let mut end: Option<i32> = None;
            while let Some(key) = tri!(map.next_key()) {
                match key {
                    Field::Start => {
                        if start.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("start"));
                        }
                        start = Some(tri!(map.next_value()));
                    }
                    Field::End => {
                        if end.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("end"));
                        }
                        end = Some(tri!(map.next_value()));
                    }
                }
            }
            let start = start.ok_or_else(|| <A::Error as Error>::missing_field("start"))?;
            let end = end.ok_or_else(|| <A::Error as Error>::missing_field("end"))?;
            Ok((start, end))
        }
    }

    let keys = vec![Field::End];
    let values = vec![2];
    let test_map = TestMap::new(keys, values);
    let visitor = TestVisitor { phantom: PhantomData };
    let _ = visitor.visit_map(test_map);
}

#[test]
#[should_panic(expected = "duplicate field `start`")]
fn test_visit_map_duplicate_start() {
    use std::marker::PhantomData;
    use std::collections::HashMap;

    struct TestMap {
        keys: Vec<Field>,
        values: Vec<i32>,
        index: usize,
    }

    impl TestMap {
        fn new(keys: Vec<Field>, values: Vec<i32>) -> Self {
            Self { keys, values, index: 0 }
        }
    }

    impl<'de> MapAccess<'de> for TestMap {
        type Error = serde::de::Error;

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
            if self.index - 1 < self.values.len() {
                let value: T = self.values[self.index - 1].try_into().map_err(|_| serde::de::Error::custom("Conversion error"))?;
                Ok(value)
            } else {
                Err(serde::de::Error::custom("No value for key"))
            }
        }
    }

    struct TestVisitor {
        phantom: PhantomData<i32>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = (i32, i32);
        fn expecting(&self, _formatter: &mut std::fmt::Formatter) -> fmt::Result {
            Ok(())
        }
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut start: Option<i32> = None;
            let mut end: Option<i32> = None;
            while let Some(key) = tri!(map.next_key()) {
                match key {
                    Field::Start => {
                        if start.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("start"));
                        }
                        start = Some(tri!(map.next_value()));
                    }
                    Field::End => {
                        if end.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("end"));
                        }
                        end = Some(tri!(map.next_value()));
                    }
                }
            }
            let start = start.ok_or_else(|| <A::Error as Error>::missing_field("start"))?;
            let end = end.ok_or_else(|| <A::Error as Error>::missing_field("end"))?;
            Ok((start, end))
        }
    }

    let keys = vec![Field::Start, Field::Start, Field::End];
    let values = vec![1, 2, 3];
    let test_map = TestMap::new(keys, values);
    let visitor = TestVisitor { phantom: PhantomData };
    let _ = visitor.visit_map(test_map);
}

