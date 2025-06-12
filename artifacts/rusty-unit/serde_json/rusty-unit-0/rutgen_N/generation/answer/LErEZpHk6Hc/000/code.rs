// Answer 0

#[test]
fn test_serialize_unit_variant() {
    let name = "SampleEnum";
    let variant_index = 0;
    let variant = "VariantA";

    let result = serialize_unit_variant(name, variant_index, variant).unwrap();
    assert_eq!(result, "VariantA");
}

#[test]
#[should_panic]
fn test_serialize_unit_variant_empty_variant() {
    let name = "SampleEnum";
    let variant_index = 0;
    let variant = "";

    let result = serialize_unit_variant(name, variant_index, variant).unwrap();
    assert_eq!(result, "");
}

