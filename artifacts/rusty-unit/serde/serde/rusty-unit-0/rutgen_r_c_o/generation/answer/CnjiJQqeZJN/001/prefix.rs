// Answer 0

#[test]
fn test_tuple_variant_with_bool() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Bool(false)),
        err: PhantomData,
    };
    deserializer.tuple_variant(1, MyVisitor);
}

#[test]
fn test_tuple_variant_with_u8() {
    let deserializer = VariantDeserializer {
        value: Some(Content::U8(255)),
        err: PhantomData,
    };
    deserializer.tuple_variant(1, MyVisitor);
}

#[test]
fn test_tuple_variant_with_i64() {
    let deserializer = VariantDeserializer {
        value: Some(Content::I64(-1)),
        err: PhantomData,
    };
    deserializer.tuple_variant(1, MyVisitor);
}

#[test]
fn test_tuple_variant_with_f32() {
    let deserializer = VariantDeserializer {
        value: Some(Content::F32(3.14)),
        err: PhantomData,
    };
    deserializer.tuple_variant(1, MyVisitor);
}

#[test]
fn test_tuple_variant_with_char() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Char('x')),
        err: PhantomData,
    };
    deserializer.tuple_variant(1, MyVisitor);
}

struct MyVisitor;

impl<'de> de::Visitor<'de> for MyVisitor {
    type Value = ();
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected tuple variant")
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }
}

