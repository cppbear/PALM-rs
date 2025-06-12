// Answer 0

#[test]
fn test_serialize_i64_negative_max() {
    let serializer = TaggedSerializer {
        type_ident: "Test",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: // initialize with a concrete serializer,
    };
    let _ = serializer.serialize_i64(-9223372036854775808);
}

#[test]
fn test_serialize_i64_negative_small() {
    let serializer = TaggedSerializer {
        type_ident: "Test",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: // initialize with a concrete serializer,
    };
    let _ = serializer.serialize_i64(-1);
}

#[test]
fn test_serialize_i64_zero() {
    let serializer = TaggedSerializer {
        type_ident: "Test",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: // initialize with a concrete serializer,
    };
    let _ = serializer.serialize_i64(0);
}

#[test]
fn test_serialize_i64_positive_small() {
    let serializer = TaggedSerializer {
        type_ident: "Test",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: // initialize with a concrete serializer,
    };
    let _ = serializer.serialize_i64(1);
}

#[test]
fn test_serialize_i64_positive_max() {
    let serializer = TaggedSerializer {
        type_ident: "Test",
        variant_ident: "TestVariant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: // initialize with a concrete serializer,
    };
    let _ = serializer.serialize_i64(9223372036854775807);
}

