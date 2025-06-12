// Answer 0

#[test]
fn test_visit_seq() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an array of integers")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    struct TestSeqAccess {
        values: Vec<Option<i32>>,
        index: usize,
    }

    impl<'de> serde::de::SeqAccess<'de> for TestSeqAccess {
        type Error = serde::de::value::Error;

        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: serde::de::Deserialize<'de>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index].take();
                self.index += 1;
                Ok(value.map(|v| serde::de::Deserialize::deserialize(serde::de::value::Deserializer::new(v)).unwrap()))
            } else {
                Ok(None)
            }
        }
    }

    let seq_access = TestSeqAccess { values: vec![Some(1), Some(2), Some(3)], index: 0 };
    let visitor = TestVisitor;
    let result: Result<Vec<i32>, _> = visitor.visit_seq(seq_access);
    
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

