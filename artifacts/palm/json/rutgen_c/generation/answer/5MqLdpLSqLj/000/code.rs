// Answer 0

#[test]
fn test_serialize_newtype_struct_string() {
    struct TestData {
        value: String,
    }

    impl Serialize for TestData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.value)
        }
    }

    let serializer = MapKeySerializer;
    let data = TestData {
        value: "test".to_string(),
    };

    let result = serializer.serialize_newtype_struct("TestData", &data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_serialize_newtype_struct_i32() {
    struct TestData {
        value: i32,
    }

    impl Serialize for TestData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_i32(self.value)
        }
    }

    let serializer = MapKeySerializer;
    let data = TestData { value: 42 };

    let result = serializer.serialize_newtype_struct("TestData", &data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "42");
}

#[test]
fn test_serialize_newtype_struct_bool() {
    struct TestData {
        value: bool,
    }

    impl Serialize for TestData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_bool(self.value)
        }
    }

    let serializer = MapKeySerializer;
    let data = TestData { value: true };

    let result = serializer.serialize_newtype_struct("TestData", &data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "true");
}

#[should_panic]
#[test]
fn test_serialize_newtype_struct_invalid() {
    struct InvalidData;

    impl Serialize for InvalidData {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            panic!("This should not be called")
        }
    }

    let serializer = MapKeySerializer;
    let data = InvalidData;

    let result = serializer.serialize_newtype_struct("InvalidData", &data);
    assert!(result.is_err());
}

