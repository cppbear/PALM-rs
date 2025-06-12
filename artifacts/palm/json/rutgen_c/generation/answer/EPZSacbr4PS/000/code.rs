// Answer 0

#[test]
fn test_unit_variant_access_new() {
    struct MockReader;

    let mut deserializer = Deserializer {
        read: MockReader,
        scratch: Vec::new(),
        remaining_depth: 16,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let unit_variant_access = UnitVariantAccess::new(&mut deserializer);

    assert_eq!(std::ptr::addr_of!(unit_variant_access.de), std::ptr::addr_of!(&mut deserializer));
}

