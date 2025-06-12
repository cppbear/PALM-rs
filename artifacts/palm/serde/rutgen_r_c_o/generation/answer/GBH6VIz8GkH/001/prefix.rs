// Answer 0

#[test]
fn test_deserialize_content_bool() {
    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_u8() {
    let content = Content::U8(255);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_u16() {
    let content = Content::U16(65535);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_i8() {
    let content = Content::I8(-128);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_i16() {
    let content = Content::I16(-32768);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_i32() {
    let content = Content::I32(-2147483648);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_f32() {
    let content = Content::F32(0.0);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_f64() {
    let content = Content::F64(0.0);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_char() {
    let content = Content::Char('a');
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_string() {
    let content = Content::String(String::from("valid string"));
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_bytes() {
    let content = Content::Bytes(vec![0u8; 512]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_seq() {
    let content = Content::Seq(vec![Content::Bool(true), Content::I32(1)]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

#[test]
fn test_deserialize_content_map() {
    let content = Content::Map(vec![(Content::String(String::from("key")), Content::U32(10))]);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    let _ = deserializer.__deserialize_content(actually_private::T, visitor);
}

struct MyVisitor;

impl<'de> Visitor<'de> for MyVisitor {
    type Value = Content<'de>;

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Content::Bool(value))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
        Ok(Content::U8(value))
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
        Ok(Content::U16(value))
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E> {
        Ok(Content::I8(value))
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> {
        Ok(Content::I16(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> {
        Ok(Content::I32(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> {
        Ok(Content::F32(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
        Ok(Content::F64(value))
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E> {
        Ok(Content::Char(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Content::String(value.to_string()))
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E> {
        Ok(Content::Bytes(value.to_vec()))
    }

    fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, Self::Error>
    where
        V: Visitor<'de>, {
        unimplemented!()
    }

    fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, Self::Error>
    where
        V: Visitor<'de>, {
        unimplemented!()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Content::Unit)
    }
}

