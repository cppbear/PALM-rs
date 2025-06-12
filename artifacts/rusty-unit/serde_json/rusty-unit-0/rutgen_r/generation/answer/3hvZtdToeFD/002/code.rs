// Answer 0

#[derive(Default)]
struct MockDeserializer {
    parse_result: Result<(), ()>,
}

impl MockDeserializer {
    fn parse_object_colon(&mut self) -> Result<(), ()> {
        self.parse_result.clone()
    }
}

struct Seed {}

impl<'de> de::DeserializeSeed<'de> for Seed {
    type Value = i32;

    fn deserialize<A>(self, _deserializer: A) -> Result<Self::Value>
    where
        A: de::Deserializer<'de>,
    {
        Ok(42)
    }
}

#[test]
fn test_next_value_seed_success() {
    let mut de = MockDeserializer {
        parse_result: Ok(()),
    };

    let seed = Seed {};
    let result = next_value_seed(&mut de, seed);

    assert_eq!(result.unwrap(), 42);
}

#[test]
#[should_panic]
fn test_next_value_seed_parse_error() {
    let mut de = MockDeserializer {
        parse_result: Err(()),
    };

    let seed = Seed {};
    let _ = next_value_seed(&mut de, seed);
}

