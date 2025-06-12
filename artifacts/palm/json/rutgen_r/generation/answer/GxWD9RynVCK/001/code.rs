// Answer 0

#[test]
fn test_deserialize_tuple_struct_valid() {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::marker::PhantomData;

    struct DummyVisitor {
        value: Vec<i32>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut out = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                out.push(value);
            }
            Ok(out)
        }
    }

    struct DummyDeserializer {
        data: Vec<i32>,
        index: usize,
    }

    impl DummyDeserializer {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
        }
        
        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de> {
            let data_len = self.data.len();
            let mut seq_access = self.data;

            let seq_visitor = DummyVisitor { value: seq_access };

            visitor.visit_seq(seq_visitor)
        }
    }

    let deserializer = DummyDeserializer::new(vec![1, 2, 3]);
    let result: Result<Vec<i32>, de::Error> = deserializer.deserialize_tuple_struct("Dummy", 3, DummyVisitor { value: Vec::new() });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
#[should_panic]
fn test_deserialize_tuple_struct_empty_sequence() {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::marker::PhantomData;

    struct DummyVisitor {
        value: Vec<i32>,
    }

    impl<'de> Visitor<'de> for DummyVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a tuple struct")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::SeqAccess<'de>,
        {
            let mut out = Vec::new();
            while let Some(value) = seq.next_element::<i32>()? {
                out.push(value);
            }
            Ok(out)
        }
    }

    struct DummyDeserializer {
        data: Vec<i32>,
        index: usize,
    }

    impl DummyDeserializer {
        fn new(data: Vec<i32>) -> Self {
            Self { data, index: 0 }
        }
        
        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de> {
            let seq_visitor = DummyVisitor { value: self.data };

            visitor.visit_seq(seq_visitor)
        }
    }

    let deserializer = DummyDeserializer::new(vec![]);
    let _result: Result<Vec<i32>, de::Error> = deserializer.deserialize_tuple_struct("Dummy", 0, DummyVisitor { value: Vec::new() });
}

