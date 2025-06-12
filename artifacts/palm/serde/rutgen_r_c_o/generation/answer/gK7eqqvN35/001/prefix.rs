// Answer 0

#[test]
fn test_serialize_f64_positive() {
    let serializer = TaggedSerializer::<MySerializer> {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "test_tag",
        variant_name: "test_variant",
        delegate: MySerializer::new(),
    };
    let _ = serializer.serialize_f64(1.0e+300);
}

#[test]
fn test_serialize_f64_negative() {
    let serializer = TaggedSerializer::<MySerializer> {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "test_tag",
        variant_name: "test_variant",
        delegate: MySerializer::new(),
    };
    let _ = serializer.serialize_f64(-1.0e+300);
}

#[test]
fn test_serialize_f64_nan() {
    let serializer = TaggedSerializer::<MySerializer> {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "test_tag",
        variant_name: "test_variant",
        delegate: MySerializer::new(),
    };
    let _ = serializer.serialize_f64(f64::NAN);
}

#[test]
fn test_serialize_f64_infinity() {
    let serializer = TaggedSerializer::<MySerializer> {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "test_tag",
        variant_name: "test_variant",
        delegate: MySerializer::new(),
    };
    let _ = serializer.serialize_f64(f64::INFINITY);
}

#[test]
fn test_serialize_f64_negative_infinity() {
    let serializer = TaggedSerializer::<MySerializer> {
        type_ident: "TestType",
        variant_ident: "TestVariant",
        tag: "test_tag",
        variant_name: "test_variant",
        delegate: MySerializer::new(),
    };
    let _ = serializer.serialize_f64(f64::NEG_INFINITY);
}

