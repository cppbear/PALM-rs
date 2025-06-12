// Answer 0

#[test]
fn test_variant_access_new() {
    struct MockDeserializer {
        remaining_depth: u8,
    }

    impl MockDeserializer {
        fn new() -> Self {
            MockDeserializer { remaining_depth: 10 }
        }
    }

    let mut deserializer = MockDeserializer::new();
    let variant_access = VariantAccess::new(&mut deserializer);
    
    assert_eq!(std::ptr::addr_of!(variant_access.de), std::ptr::addr_of!(&mut deserializer));
}

#[test]
fn test_variant_access_new_with_high_depth() {
    struct MockDeserializer {
        remaining_depth: u8,
    }

    impl MockDeserializer {
        fn new() -> Self {
            MockDeserializer { remaining_depth: 255 }
        }
    }

    let mut deserializer = MockDeserializer::new();
    let variant_access = VariantAccess::new(&mut deserializer);
    
    assert_eq!(std::ptr::addr_of!(variant_access.de), std::ptr::addr_of!(&mut deserializer));
}

