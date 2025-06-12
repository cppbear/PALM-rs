// Answer 0

#[test]
fn test_serialize_newtype_struct_string() {
    struct TestStruct;

    impl Serialize for TestStruct {
        fn serialize<S>(&self, serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str("test")
        }
    }

    let mut writer = Vec::new();
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter: CompactFormatter,
    };
    
    let test_value = TestStruct;
    let result = serializer.serialize_newtype_struct("test_struct", &test_value);
    assert!(result.is_ok());
    assert_eq!(writer.as_slice(), b"\"test\"");
}

#[test]
fn test_serialize_newtype_struct_int() {
    struct IntWrapper(i32);

    impl Serialize for IntWrapper {
        fn serialize<S>(&self, serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            serializer.serialize_i32(self.0)
        }
    }

    let mut writer = Vec::new();
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter: CompactFormatter,
    };
    
    let test_value = IntWrapper(42);
    let result = serializer.serialize_newtype_struct("int_wrapper", &test_value);
    assert!(result.is_ok());
    assert_eq!(writer.as_slice(), b"42");
}

#[should_panic]
#[test]
fn test_serialize_newtype_struct_invalid() {
    struct Invalid;

    impl Serialize for Invalid {
        fn serialize<S>(&self, _serializer: S) -> Result<()>
        where
            S: ser::Serializer,
        {
            Err(Error::new("serialization error"))
        }
    }

    let mut writer = Vec::new();
    let mut serializer = Serializer {
        writer: &mut writer,
        formatter: CompactFormatter,
    };
    
    let test_value = Invalid;
    let _ = serializer.serialize_newtype_struct("invalid", &test_value);
}

