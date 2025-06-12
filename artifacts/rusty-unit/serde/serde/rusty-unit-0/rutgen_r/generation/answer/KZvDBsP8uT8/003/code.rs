// Answer 0

#[derive(Debug)]
struct MockKey;

impl<'de> de::DeserializeSeed<'de> for MockKey {
    type Value = String;

    fn deserialize(self, deserializer: &mut dyn de::Deserializer<'de>) -> Result<Self::Value, de::Error> {
        let key: String = deserializer.deserialize_str(StringDeserializer)?.into();
        Ok(key)
    }
}

#[derive(Debug)]
struct MockValue;

impl<'de> de::DeserializeSeed<'de> for MockValue {
    type Value = i32;

    fn deserialize(self, deserializer: &mut dyn de::Deserializer<'de>) -> Result<Self::Value, de::Error> {
        let value: i32 = deserializer.deserialize_i32(IntDeserializer)?.into();
        Ok(value)
    }
}

struct MockDeserializer {
    pairs: Vec<(String, i32)>,
    index: usize,
}

impl MockDeserializer {
    fn new(pairs: Vec<(String, i32)>) -> Self {
        Self { pairs, index: 0 }
    }

    fn next_pair(&mut self) -> Option<(String, i32)> {
        if self.index < self.pairs.len() {
            let pair = self.pairs[self.index].clone();
            self.index += 1;
            Some(pair)
        } else {
            None
        }
    }
}

#[test]
fn test_next_entry_seed_success() {
    let mut deserializer = MockDeserializer::new(vec![("key1".to_string(), 42)]);
    let kseed = MockKey;
    let vseed = MockValue;

    let result = deserializer.next_entry_seed(kseed, vseed);
    assert_eq!(result, Ok(Some(("key1".to_string(), 42))));
}

