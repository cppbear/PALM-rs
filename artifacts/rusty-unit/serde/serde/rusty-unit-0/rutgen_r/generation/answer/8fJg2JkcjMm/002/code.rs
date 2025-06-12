// Answer 0

#[derive(Debug)]
struct MockVisitor {
    values: Vec<i32>,
}

impl<'de> Visitor<'de> for MockVisitor {
    type Value = Vec<i32>;

    fn visit_seq<A>(self, _: A) -> Result<Self::Value, E>
    where
        A: SeqAccess<'de>,
    {
        Ok(self.values.clone())
    }

    fn visit_bool(self, _: bool) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_u8(self, _: u8) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_u16(self, _: u16) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_u32(self, _: u32) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_u64(self, _: u64) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_i8(self, _: i8) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_i16(self, _: i16) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_i32(self, _: i32) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_i64(self, _: i64) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_f32(self, _: f32) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_f64(self, _: f64) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_char(self, _: char) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_str(self, _: &str) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_borrowed_str(self, _: &'de str) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_bytes(self, _: &[u8]) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_borrowed_bytes(self, _: &'de [u8]) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_unit(self) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_none(self) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_some<T>(self, _: T) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
    fn visit_newtype_struct<T>(self, _: T) -> Result<Self::Value, E> { Err(E::custom("invalid type")) }
}

#[test]
fn test_deserialize_any_with_seq() {
    let content = Content::Seq(vec![1, 2, 3]);
    let deserializer = Deserializer::from_content(content);
    let visitor = MockVisitor { values: vec![1, 2, 3] };

    let result: Result<Vec<i32>, E> = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_any_with_empty_seq() {
    let content = Content::Seq(vec![]);
    let deserializer = Deserializer::from_content(content);
    let visitor = MockVisitor { values: vec![] };

    let result: Result<Vec<i32>, E> = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), vec![]);
}

