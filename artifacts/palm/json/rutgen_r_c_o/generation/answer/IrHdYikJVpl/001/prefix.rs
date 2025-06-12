// Answer 0

#[test]
fn test_deserialize_bytes_empty() {
    let value = Value::Bytes(vec![]);
    let visitor = MyVisitor {};
    let result = value.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_small() {
    let value = Value::Bytes(vec![0u8; 1]); // 1 byte
    let visitor = MyVisitor {};
    let result = value.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_medium() {
    let value = Value::Bytes(vec![0u8; 32768]); // 32 KB
    let visitor = MyVisitor {};
    let result = value.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_large() {
    let value = Value::Bytes(vec![0u8; 65536]); // 64 KB
    let visitor = MyVisitor {};
    let result = value.deserialize_bytes(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_bytes_overflow() {
    let value = Value::Bytes(vec![0u8; 65537]); // 65 KB (above limit)
    let visitor = MyVisitor {};
    let result = value.deserialize_bytes(visitor);
}

struct MyVisitor;

impl<'de> Visitor<'de> for MyVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a byte sequence")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
        Ok(value.to_vec())
    }
    
    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E> {
        Ok(value)
    }
}

