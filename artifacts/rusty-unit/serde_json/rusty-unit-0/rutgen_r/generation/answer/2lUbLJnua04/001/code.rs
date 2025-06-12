// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    struct Serializer;

    impl Serializer {
        fn serialize_str(&self, _variant: &str) -> Result<(), &'static str> {
            Ok(())
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<(), &'static str> {
            self.serialize_str(variant)
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_unit_variant("TestEnum", 0, "VariantA");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_unit_variant_panic_empty_variant() {
    struct Serializer;

    impl Serializer {
        fn serialize_str(&self, variant: &str) -> Result<(), &'static str> {
            assert!(!variant.is_empty(), "Variant cannot be empty");
            Ok(())
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<(), &'static str> {
            self.serialize_str(variant)
        }
    }

    let serializer = Serializer;
    let _ = serializer.serialize_unit_variant("TestEnum", 0, "");
}

#[test]
fn test_serialize_unit_variant_with_index() {
    struct Serializer;

    impl Serializer {
        fn serialize_str(&self, variant: &str) -> Result<(), &'static str> {
            match variant {
                "Variant1" | "Variant2" => Ok(()),
                _ => Err("Invalid variant"),
            }
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<(), &'static str> {
            self.serialize_str(variant)
        }
    }

    let serializer = Serializer;

    let result1 = serializer.serialize_unit_variant("TestEnum", 1, "Variant1");
    assert!(result1.is_ok());

    let result2 = serializer.serialize_unit_variant("TestEnum", 2, "Variant2");
    assert!(result2.is_ok());

    let result3 = serializer.serialize_unit_variant("TestEnum", 3, "Variant3");
    assert!(result3.is_err());
}

