// Answer 0

#[test]
fn test_next_value_seed_bool() {
    let value = Value::Bool(true);
    let mut deserializer = MapRefDeserializer {
        iter: std::iter::empty().into_iter(),
        value: Some(&value),
    };
    struct BoolSeed;
    impl<'de> DeserializeSeed<'de> for BoolSeed {
        type Value = bool;
        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            bool::deserialize(deserializer)
        }
    }
    let seed = BoolSeed;
    let _ = deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_number() {
    let value = Value::Number(Number::from(1));
    let mut deserializer = MapRefDeserializer {
        iter: std::iter::empty().into_iter(),
        value: Some(&value),
    };
    struct NumberSeed;
    impl<'de> DeserializeSeed<'de> for NumberSeed {
        type Value = Number;
        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            Number::deserialize(deserializer)
        }
    }
    let seed = NumberSeed;
    let _ = deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_string() {
    let value = Value::String(String::from("test"));
    let mut deserializer = MapRefDeserializer {
        iter: std::iter::empty().into_iter(),
        value: Some(&value),
    };
    struct StringSeed;
    impl<'de> DeserializeSeed<'de> for StringSeed {
        type Value = String;
        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            String::deserialize(deserializer)
        }
    }
    let seed = StringSeed;
    let _ = deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_array() {
    let value = Value::Array(vec![Value::Bool(false)]);
    let mut deserializer = MapRefDeserializer {
        iter: std::iter::empty().into_iter(),
        value: Some(&value),
    };
    struct ArraySeed;
    impl<'de> DeserializeSeed<'de> for ArraySeed {
        type Value = Vec<Value>;
        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            Vec::<Value>::deserialize(deserializer)
        }
    }
    let seed = ArraySeed;
    let _ = deserializer.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_object() {
    let value = Value::Object(Map::new());
    let mut deserializer = MapRefDeserializer {
        iter: std::iter::empty().into_iter(),
        value: Some(&value),
    };
    struct ObjectSeed;
    impl<'de> DeserializeSeed<'de> for ObjectSeed {
        type Value = Map<String, Value>;
        fn deserialize<DES>(self, deserializer: DES) -> Result<Self::Value, Error>
        where
            DES: Deserializer<'de>,
        {
            Map::<String, Value>::deserialize(deserializer)
        }
    }
    let seed = ObjectSeed;
    let _ = deserializer.next_value_seed(seed);
}

