// Answer 0

#[test]
fn test_newtype_variant_seed_bool() {
    struct BoolSeed;

    impl<'de> DeserializeSeed<'de> for BoolSeed {
        type Value = bool;

        fn deserialize<D>(self, deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bool(self)
        }
    }

    let value = Value::Bool(true);
    let deserializer = VariantRefDeserializer { value: Some(&value) };
    let _ = deserializer.newtype_variant_seed(BoolSeed);
}

#[test]
fn test_newtype_variant_seed_number() {
    struct NumberSeed;

    impl<'de> DeserializeSeed<'de> for NumberSeed {
        type Value = i64;

        fn deserialize<D>(self, deserializer: D) -> Result<i64, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_i64(self)
        }
    }

    let value = Value::Number(Number::from(0));
    let deserializer = VariantRefDeserializer { value: Some(&value) };
    let _ = deserializer.newtype_variant_seed(NumberSeed);
}

#[test]
fn test_newtype_variant_seed_string() {
    struct StringSeed;

    impl<'de> DeserializeSeed<'de> for StringSeed {
        type Value = String;

        fn deserialize<D>(self, deserializer: D) -> Result<String, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_string(self)
        }
    }

    let value = Value::String(String::from(""));
    let deserializer = VariantRefDeserializer { value: Some(&value) };
    let _ = deserializer.newtype_variant_seed(StringSeed);
}

#[test]
fn test_newtype_variant_seed_array() {
    struct ArraySeed;

    impl<'de> DeserializeSeed<'de> for ArraySeed {
        type Value = Vec<Value>;

        fn deserialize<D>(self, deserializer: D) -> Result<Vec<Value>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(self)
        }
    }

    let value = Value::Array(vec![Value::Null]);
    let deserializer = VariantRefDeserializer { value: Some(&value) };
    let _ = deserializer.newtype_variant_seed(ArraySeed);
}

#[test]
fn test_newtype_variant_seed_object() {
    struct ObjectSeed;

    impl<'de> DeserializeSeed<'de> for ObjectSeed {
        type Value = Map<String, Value>;

        fn deserialize<D>(self, deserializer: D) -> Result<Map<String, Value>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(self)
        }
    }

    let value = Value::Object(Map::new());
    let deserializer = VariantRefDeserializer { value: Some(&value) };
    let _ = deserializer.newtype_variant_seed(ObjectSeed);
}

