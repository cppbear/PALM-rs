// Answer 0

#[test]
fn test_serialize_unit_variant_valid_input() {
    let name = "TestEnum";
    let variant_index = 0;
    let variant = "VariantA";

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

#[test]
fn test_serialize_unit_variant_another_variant() {
    let name = "TestEnum";
    let variant_index = 1;
    let variant = "VariantB";

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

#[test]
fn test_serialize_unit_variant_empty_variant() {
    let name = "TestEnum";
    let variant_index = 2;
    let variant = "";

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

#[test]
fn test_serialize_unit_variant_long_variant() {
    let name = "TestEnum";
    let variant_index = 3;
    let variant = "ThisIsAVeryLongVariantName";

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

