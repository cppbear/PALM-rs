// Answer 0

#[test]
fn test_deserialize_char_a() {
    let value = Value::Char('a');
    let visitor = MockVisitor {};
    value.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_null() {
    let value = Value::Char('\0');
    let visitor = MockVisitor {};
    value.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_unicode() {
    let value = Value::Char('𝒜');
    let visitor = MockVisitor {};
    value.deserialize_char(visitor);
}

#[test]
fn test_deserialize_char_emoji() {
    let value = Value::Char('😊');
    let visitor = MockVisitor {};
    value.deserialize_char(visitor);
}

struct MockVisitor;

impl<'de> Visitor<'de> for MockVisitor {
    type Value = char;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a char")
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E> {
        Ok(value)
    }

    forward_to_deserialize_any! {
        bool i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 str string bytes byte_buf option unit
        seq tuple tuple_struct map struct identifier ignored_any
    }
}

