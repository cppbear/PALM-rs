// Answer 0

#[test]
fn test_unit_variant_bool() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Bool(true)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_u8() {
    let deserializer = VariantDeserializer {
        value: Some(Content::U8(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_u16() {
    let deserializer = VariantDeserializer {
        value: Some(Content::U16(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_u32() {
    let deserializer = VariantDeserializer {
        value: Some(Content::U32(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_u64() {
    let deserializer = VariantDeserializer {
        value: Some(Content::U64(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_i8() {
    let deserializer = VariantDeserializer {
        value: Some(Content::I8(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_i16() {
    let deserializer = VariantDeserializer {
        value: Some(Content::I16(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_i32() {
    let deserializer = VariantDeserializer {
        value: Some(Content::I32(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_i64() {
    let deserializer = VariantDeserializer {
        value: Some(Content::I64(0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_f32() {
    let deserializer = VariantDeserializer {
        value: Some(Content::F32(0.0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_f64() {
    let deserializer = VariantDeserializer {
        value: Some(Content::F64(0.0)),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_char() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Char('a')),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_string() {
    let deserializer = VariantDeserializer {
        value: Some(Content::String(String::from(""))),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_bytes() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Bytes(vec![])),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_some() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Some(Box::new(Content::Bool(false)))),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

#[test]
fn test_unit_variant_unit() {
    let deserializer = VariantDeserializer {
        value: Some(Content::Unit),
        err: PhantomData,
    };
    deserializer.unit_variant();
}

