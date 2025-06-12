// Answer 0

#[test]
fn test_tuple_variant_bool() {
    let deserializer = VariantRefDeserializer {
        value: Some(&Content::Bool(true)),
        err: PhantomData,
    };
    deserializer.tuple_variant(0, DummyVisitor);
}

#[test]
fn test_tuple_variant_f32() {
    let deserializer = VariantRefDeserializer {
        value: Some(&Content::F32(3.14)),
        err: PhantomData,
    };
    deserializer.tuple_variant(0, DummyVisitor);
}

#[test]
fn test_tuple_variant_char() {
    let deserializer = VariantRefDeserializer {
        value: Some(&Content::Char('a')),
        err: PhantomData,
    };
    deserializer.tuple_variant(0, DummyVisitor);
}

#[test]
fn test_tuple_variant_u8() {
    let deserializer = VariantRefDeserializer {
        value: Some(&Content::U8(255)),
        err: PhantomData,
    };
    deserializer.tuple_variant(0, DummyVisitor);
}

#[test]
fn test_tuple_variant_i32() {
    let deserializer = VariantRefDeserializer {
        value: Some(&Content::I32(-42)),
        err: PhantomData,
    };
    deserializer.tuple_variant(0, DummyVisitor);
}

#[test]
fn test_tuple_variant_string() {
    let deserializer = VariantRefDeserializer {
        value: Some(&Content::String("test".to_string())),
        err: PhantomData,
    };
    deserializer.tuple_variant(0, DummyVisitor);
}

