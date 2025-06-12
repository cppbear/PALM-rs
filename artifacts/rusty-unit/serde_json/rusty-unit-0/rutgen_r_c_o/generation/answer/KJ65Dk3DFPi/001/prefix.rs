// Answer 0

#[test]
fn test_deserialize_bytes_zero_length() {
    let visitor = DummyVisitor {};
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_max_length() {
    let visitor = DummyVisitor {};
    let data = vec![0u8; 4096]; // 4096 bytes of data
    let mut deserializer = Deserializer {
        read: SliceRead::new(&data),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_some_length() {
    let visitor = DummyVisitor {};
    let data = vec![1u8; 1024]; // 1024 bytes of data
    let mut deserializer = Deserializer {
        read: SliceRead::new(&data),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_partial_read() {
    let visitor = DummyVisitor {};
    let data = vec![2u8; 2048]; // 2048 bytes of data
    let mut deserializer = Deserializer {
        read: SliceRead::new(&data),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.deserialize_bytes(visitor);
}

struct DummyVisitor;

impl<'de> de::Visitor<'de> for DummyVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte array")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
        Ok(value.to_vec())
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> {
        Ok(value)
    }
}

