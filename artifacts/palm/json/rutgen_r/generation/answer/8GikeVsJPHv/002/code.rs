// Answer 0

#[test]
fn test_next_key_seed_with_valid_key() {
    struct DummySeed;

    impl<'de> de::DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<Access>(self, _: Access) -> Result<String> {
            Ok("key".to_string())
        }
    }

    struct DummyDeserializer<'de> {
        input: &'de [u8],
        index: usize,
    }

    impl<'de> DummyDeserializer<'de> {
        fn parse_whitespace(&mut self) -> Option<u8> {
            while self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                if !byte.is_ascii_whitespace() {
                    return Some(byte);
                }
            }
            None
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("peek error")
        }
    }

    impl<'de> de::Deserializer<'de> for DummyDeserializer<'de> {
        type Error = Error;

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.visit_map(MapAccess {
                de: self,
                first: true,
            })
        }
    }

    struct MapAccess<'a, R: Read<'de> + 'a> {
        de: &'a mut R,
        first: bool,
    }

    let input_data: &[u8] = b"{\"key\": 1}";
    let mut deserializer = DummyDeserializer {
        input: input_data,
        index: 0,
    };
    let seed = DummySeed;

    let result = deserializer.next_key_seed(seed).unwrap();
    assert_eq!(result, Some("key".to_string()));
}

#[test]
fn test_next_key_seed_with_empty_object() {
    struct DummySeed;

    impl<'de> de::DeserializeSeed<'de> for DummySeed {
        type Value = String;

        fn deserialize<Access>(self, _: Access) -> Result<String> {
            Ok("key".to_string())
        }
    }

    struct DummyDeserializer<'de> {
        input: &'de [u8],
        index: usize,
    }

    impl<'de> DummyDeserializer<'de> {
        fn parse_whitespace(&mut self) -> Option<u8> {
            while self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                if !byte.is_ascii_whitespace() {
                    return Some(byte);
                }
            }
            None
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("peek error")
        }
    }

    impl<'de> de::Deserializer<'de> for DummyDeserializer<'de> {
        type Error = Error;

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.visit_map(MapAccess {
                de: self,
                first: true,
            })
        }
    }

    struct MapAccess<'a, R: Read<'de> + 'a> {
        de: &'a mut R,
        first: bool,
    }

    let input_data: &[u8] = b"{}";
    let mut deserializer = DummyDeserializer {
        input: input_data,
        index: 0,
    };
    let seed = DummySeed;

    let result = deserializer.next_key_seed(seed).unwrap();
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_next_key_seed_with_invalid_key() {
    struct InvalidSeed;

    impl<'de> de::DeserializeSeed<'de> for InvalidSeed {
        type Value = String;

        fn deserialize<Access>(self, _: Access) -> Result<String> {
            Err(Error::custom("invalid key"))
        }
    }

    struct DummyDeserializer<'de> {
        input: &'de [u8],
        index: usize,
    }

    impl<'de> DummyDeserializer<'de> {
        fn parse_whitespace(&mut self) -> Option<u8> {
            while self.index < self.input.len() {
                let byte = self.input[self.index];
                self.index += 1;
                if !byte.is_ascii_whitespace() {
                    return Some(byte);
                }
            }
            None
        }

        fn peek_error(&self, _: ErrorCode) -> Error {
            Error::custom("peek error")
        }
    }

    impl<'de> de::Deserializer<'de> for DummyDeserializer<'de> {
        type Error = Error;

        fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            visitor.visit_map(MapAccess {
                de: self,
                first: true,
            })
        }
    }

    struct MapAccess<'a, R: Read<'de> + 'a> {
        de: &'a mut R,
        first: bool,
    }

    let input_data: &[u8] = b"{\"invalid_key\": 1}";
    let mut deserializer = DummyDeserializer {
        input: input_data,
        index: 0,
    };
    let seed = InvalidSeed;

    let _ = deserializer.next_key_seed(seed).unwrap();
}

