// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    let variant = "MyVariant";
    let name = "MyEnum";
    let variant_index = 0;

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

#[test]
fn test_serialize_unit_variant_empty_variant() {
    let variant = "";
    let name = "MyEnum";
    let variant_index = 0;

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

#[test]
fn test_serialize_unit_variant_long_variant() {
    let variant = "This_is_a_very_long_variant_name_to_test_the_edges";
    let name = "MyEnum";
    let variant_index = 1;

    let result = serialize_unit_variant(name, variant_index, variant);
    assert_eq!(result, Ok(variant.to_owned()));
}

