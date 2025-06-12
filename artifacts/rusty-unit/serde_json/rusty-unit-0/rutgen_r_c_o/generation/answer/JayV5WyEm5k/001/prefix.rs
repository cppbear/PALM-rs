// Answer 0

#[test]
fn test_variant_seed_valid_string() {
    let variant = String::from("valid_variant");
    let value = Some(Value::String(String::from("some_value")));
    let enum_deserializer = EnumDeserializer { variant, value };
    let seed = MockSeed { expected: "valid_variant".to_owned() };
    let _ = enum_deserializer.variant_seed(seed);
}

#[test]
fn test_variant_seed_empty_variant() {
    let variant = String::from("");
    let value = Some(Value::String(String::from("some_value")));
    let enum_deserializer = EnumDeserializer { variant, value };
    let seed = MockSeed { expected: "" };
    let _ = enum_deserializer.variant_seed(seed);
}

#[test]
fn test_variant_seed_large_variant() {
    let variant = "a".repeat(255);
    let value = Some(Value::String(String::from("some_value")));
    let enum_deserializer = EnumDeserializer { variant, value };
    let seed = MockSeed { expected: "a".repeat(255) };
    let _ = enum_deserializer.variant_seed(seed);
}

#[test]
#[should_panic]
fn test_variant_seed_invalid_variant() {
    let variant = String::from("invalid_variant");
    let value = Some(Value::String(String::from("some_value")));
    let enum_deserializer = EnumDeserializer { variant, value };
    let seed = MockSeed { expected: "non_matching_variant".to_owned() };
    let _ = enum_deserializer.variant_seed(seed);
}

#[test]
fn test_variant_seed_null_value() {
    let variant = String::from("valid_variant");
    let value = None;
    let enum_deserializer = EnumDeserializer { variant, value };
    let seed = MockSeed { expected: "valid_variant".to_owned() };
    let _ = enum_deserializer.variant_seed(seed);
}

struct MockSeed {
    expected: String,
}

impl<'de> DeserializeSeed<'de> for MockSeed {
    type Value = String;
    
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized: String = Deserialize::deserialize(deserializer)?;
        if deserialized == self.expected {
            Ok(deserialized)
        } else {
            Err(de::Error::custom("Invalid variant"))
        }
    }
}

