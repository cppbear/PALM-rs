// Answer 0

#[derive(Debug)]
struct TestVisitor {
    count: usize,
    values: Vec<i32>,
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = Vec<i32>;

    fn visit_seq<A>(self, seq: &mut A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut result = Vec::new();
        while let Some(value) = seq.next_element::<i32>()? {
            result.push(value);
        }
        Ok(result)
    }
}

#[derive(Debug)]
struct TestDeserializer {
    data: Vec<i32>,
    index: usize,
}

impl TestDeserializer {
    fn new(data: Vec<i32>) -> Self {
        TestDeserializer { data, index: 0 }
    }

    fn end(&mut self) -> Result<(), serde::de::Error> {
        if self.index >= self.data.len() {
            Ok(())
        } else {
            Err(serde::de::Error::custom("Did not consume all elements"))
        }
    }
}

impl<'de> serde::Deserializer<'de> for TestDeserializer {
    type Error = serde::de::Error;

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value = visitor.visit_seq(&mut self)?;
        self.end()?;
        Ok(value)
    }

    // Other trait methods would need to be implemented, but are omitted for brevity...
}

impl<'de> serde::de::SeqAccess<'de> for TestDeserializer {
    type Error = serde::de::Error;

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: serde::de::Deserialize<'de>,
    {
        if self.index < self.data.len() {
            let value: T = serde::de::Deserialize::deserialize(&mut self.data[self.index])?;
            self.index += 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    // Other trait methods would need to be implemented, but are omitted for brevity...
}

#[test]
fn test_deserialize_seq() {
    let data = vec![1, 2, 3];
    let deserializer = TestDeserializer::new(data);
    let visitor = TestVisitor { count: 0, values: Vec::new() };

    let result: Result<Vec<i32>, _> = deserializer.deserialize_seq(visitor);
    
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

