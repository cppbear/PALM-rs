// Answer 0

#[test]
fn test_serialize_field_string() {
    struct TestStruct {
        name: String,
        value: i32,
    }

    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("TestStruct", 2)?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("value", &self.value)?;
            state.end()
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("Test"),
        map: Map::new(),
    };
    
    let value = TestStruct {
        name: String::from("example"),
        value: 42,
    };

    let result = variant.serialize_field("key1", &value);
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key1"));
}

#[test]
fn test_serialize_field_bool() {
    struct BoolStruct {
        flag: bool,
    }

    impl Serialize for BoolStruct {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("BoolStruct", 1)?;
            state.serialize_field("flag", &self.flag)?;
            state.end()
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("TestBool"),
        map: Map::new(),
    };
    
    let value = BoolStruct { flag: true };

    let result = variant.serialize_field("key2", &value);
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key2"));
}

#[test]
fn test_serialize_field_float() {
    struct FloatStruct {
        number: f64,
    }

    impl Serialize for FloatStruct {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("FloatStruct", 1)?;
            state.serialize_field("number", &self.number)?;
            state.end()
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("TestFloat"),
        map: Map::new(),
    };
    
    let value = FloatStruct { number: 3.14 };

    let result = variant.serialize_field("key3", &value);
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key3"));
}

#[test]
fn test_serialize_field_null() {
    struct NullStruct;

    impl Serialize for NullStruct {
        fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_unit()
        }
    }

    let mut variant = SerializeStructVariant {
        name: String::from("TestNull"),
        map: Map::new(),
    };

    let value = NullStruct;

    let result = variant.serialize_field("key4", &value);
    assert!(result.is_ok());
    assert!(variant.map.contains_key("key4"));
}

