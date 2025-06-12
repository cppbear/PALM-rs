// Answer 0

#[test]
fn test_unit_variant_bool_true() {
    let variant = VariantDeserializer {
        value: Some(Value::Bool(true)),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_bool_false() {
    let variant = VariantDeserializer {
        value: Some(Value::Bool(false)),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_number_zero() {
    let variant = VariantDeserializer {
        value: Some(Value::Number(Number::from(0))),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_number_one() {
    let variant = VariantDeserializer {
        value: Some(Value::Number(Number::from(1))),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_string() {
    let variant = VariantDeserializer {
        value: Some(Value::String(String::from("test"))),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_null() {
    let variant = VariantDeserializer {
        value: Some(Value::Null),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_empty_array() {
    let variant = VariantDeserializer {
        value: Some(Value::Array(Vec::new())),
    };
    let _ = variant.unit_variant();
}

#[test]
fn test_unit_variant_empty_object() {
    let variant = VariantDeserializer {
        value: Some(Value::Object(Map::new())),
    };
    let _ = variant.unit_variant();
}

