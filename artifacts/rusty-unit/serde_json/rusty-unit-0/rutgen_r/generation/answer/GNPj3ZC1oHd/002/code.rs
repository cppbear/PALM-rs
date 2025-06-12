// Answer 0

#[derive(Default)]
struct TestDe {
    next_value: Option<u8>,
}

impl TestDe {
    fn parse_whitespace(&mut self) -> Option<u8> {
        self.next_value.take()
    }

    fn peek_error(&self, _error_code: ErrorCode) -> Error {
        Error::new("Peek error")
    }

    fn eat_char(&mut self) {}
}

struct SeqAccess<'a, R: Read<'a>> {
    de: &'a mut TestDe,
    first: bool,
}

impl<'a, R: Read<'a>> SeqAccess<'a, R> {
    fn new(de: &'a mut TestDe) -> Self {
        Self { de, first: true }
    }
}

#[derive(Default)]
struct TestSeed;

impl<'de> serde::de::DeserializeSeed<'de> for TestSeed {
    type Value = u8;

    fn deserialize<T>(&self, _deserializer: T) -> Result<Self::Value>
    where
        T: serde::Deserializer<'de>,
    {
        Ok(42)
    }
}

#[test]
fn test_next_element_seed_has_next_element_true() {
    let mut de = TestDe { next_value: Some(b'1') };
    let mut seq = SeqAccess::new(&mut de);
    let seed = TestSeed;

    let result = seq.next_element_seed(seed);

    assert_eq!(result, Ok(Some(42)));
}

#[test]
fn test_next_element_seed_has_next_element_false() {
    let mut de = TestDe { next_value: Some(b']') };
    let mut seq = SeqAccess::new(&mut de);
    let seed = TestSeed;

    let result = seq.next_element_seed(seed);

    assert_eq!(result, Ok(None));
}

#[test]
#[should_panic(expected = "Peek error")]
fn test_next_element_seed_error_in_has_next_element() {
    let mut de = TestDe { next_value: None };
    let mut seq = SeqAccess::new(&mut de);
    let seed = TestSeed;

    let result = seq.next_element_seed(seed);

    match result {
        Ok(_) => panic!("Expected error, but got Ok"),
        Err(_) => (),
    }
}

#[test]
#[should_panic(expected = "Peek error")]
fn test_next_element_seed_error_during_seed_deserialize() {
    struct FailingSeed;

    impl<'de> serde::de::DeserializeSeed<'de> for FailingSeed {
        type Value = u8;

        fn deserialize<T>(&self, _deserializer: T) -> Result<Self::Value>
        where
            T: serde::Deserializer<'de>,
        {
            Err(Error::new("Deserialization failure"))
        }
    }

    let mut de = TestDe { next_value: Some(b'1') };
    let mut seq = SeqAccess::new(&mut de);
    let seed = FailingSeed;

    let result = seq.next_element_seed(seed);

    match result {
        Ok(_) => panic!("Expected error during deserialization, but got Ok"),
        Err(err) => assert_eq!(err.to_string(), "Deserialization failure"),
    }
}

