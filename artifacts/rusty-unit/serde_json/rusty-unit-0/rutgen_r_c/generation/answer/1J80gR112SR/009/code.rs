// Answer 0

#[test]
fn test_deserialize_number_valid_negative() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = i64;

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Ok(0)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Ok(0)
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>, 
        {
            Ok(0)
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"-123".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.deserialize_number(MockVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -123);
}

#[test]
fn test_deserialize_number_valid_positive() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = u64;

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Ok(0)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Ok(0)
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>, 
        {
            Ok(0)
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"456".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.deserialize_number(MockVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 456);
}

#[test]
fn test_deserialize_number_eof_error() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = i64;

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Ok(0)
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Ok(0)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Ok(0)
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>, 
        {
            Ok(0)
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.deserialize_number(MockVisitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_number_invalid_type() {
    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = i64;

        fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
            Ok(0)
        }

        fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
            Ok(0)
        }
        
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
            Ok(0)
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>, 
        {
            Ok(0)
        }
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(b"abc".to_vec()),
        scratch: Vec::new(),
        remaining_depth: 128,
    };

    let result = deserializer.deserialize_number(MockVisitor);
    assert!(result.is_err());
}

struct MockRead {
    input: Vec<u8>,
    position: usize,
}

impl MockRead {
    fn new(input: Vec<u8>) -> Self {
        MockRead { input, position: 0 }
    }
}

impl<'de> Read<'de> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position < self.input.len() {
            let byte = self.input[self.position];
            self.position += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.position < self.input.len() {
            Ok(Some(self.input[self.position]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.position += 1;
    }

    fn position(&self) -> Position {
        Position { line: 0, column: self.position as u32 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: self.position as u32 }
    }

    fn byte_offset(&self) -> usize {
        self.position
    }

    fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
        unimplemented!()
    }

    fn ignore_str(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        unimplemented!()
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        unimplemented!()
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn set_failed(&mut self, _: &mut bool) {
        unimplemented!()
    }
}

