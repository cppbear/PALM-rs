// Answer 0

#[test]
fn test_deserialize_seq_valid() {
    let input = b"[1, 2, 3]";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_empty() {
    let input = b"[]";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_invalid_leading_space() {
    let input = b" [1, 2, 3]";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_error_eof() {
    let input = b"[1, 2, 3";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

#[test]
fn test_deserialize_seq_invalid_type() {
    let input = b"{1: 2}";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

#[test]
fn test_parse_whitespace_ok() {
    let input = b"   [1, 2, 3]";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

#[test]
fn test_parse_whitespace_err() {
    let input = b"invalid";
    let mut deserializer = Deserializer::from_slice(input);
    let visitor = MyVisitor;
    let _result = deserializer.deserialize_seq(visitor);
}

struct MyVisitor;

impl<'de> de::Visitor<'de> for MyVisitor {
    type Value = Vec<i32>;

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value>
    where
        V: de::SeqAccess<'de>,
    {
        let mut result = Vec::new();
        while let Some(value) = seq.next_element()? {
            result.push(value);
        }
        Ok(result)
    }
}

