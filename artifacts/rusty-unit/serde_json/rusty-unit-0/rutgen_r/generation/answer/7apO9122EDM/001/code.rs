// Answer 0

#[test]
fn test_unit_variant_success() {
    struct TestDeserializer {
        de: serde_json::Deserializer<serde_json::de::IoRead>,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            let de = serde_json::Deserializer::from_str(input);
            TestDeserializer { de }
        }

        fn unit_variant(self) -> Result<()> {
            de::Deserialize::deserialize(self.de)
        }
    }

    let deserializer = TestDeserializer::new(r#"{"variant": "UnitVariant"}"#);
    let result = deserializer.unit_variant();
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_unit_variant_invalid_json() {
    struct TestDeserializer {
        de: serde_json::Deserializer<serde_json::de::IoRead>,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            let de = serde_json::Deserializer::from_str(input);
            TestDeserializer { de }
        }

        fn unit_variant(self) -> Result<()> {
            de::Deserialize::deserialize(self.de)
        }
    }

    let deserializer = TestDeserializer::new(r#"{"unknown_variant": "UnitVariant"}"#);
    let _ = deserializer.unit_variant(); // should panic
}

#[test]
fn test_unit_variant_empty_input() {
    struct TestDeserializer {
        de: serde_json::Deserializer<serde_json::de::IoRead>,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            let de = serde_json::Deserializer::from_str(input);
            TestDeserializer { de }
        }

        fn unit_variant(self) -> Result<()> {
            de::Deserialize::deserialize(self.de)
        }
    }

    let deserializer = TestDeserializer::new(r#"{}"#);
    let result = deserializer.unit_variant();
    assert!(result.is_err());
}

#[test]
fn test_unit_variant_with_additional_fields() {
    struct TestDeserializer {
        de: serde_json::Deserializer<serde_json::de::IoRead>,
    }

    impl TestDeserializer {
        fn new(input: &str) -> Self {
            let de = serde_json::Deserializer::from_str(input);
            TestDeserializer { de }
        }

        fn unit_variant(self) -> Result<()> {
            de::Deserialize::deserialize(self.de)
        }
    }

    let deserializer = TestDeserializer::new(r#"{"variant": "UnitVariant", "extra": 123}"#);
    let result = deserializer.unit_variant();
    assert!(result.is_ok());
}

