// Answer 0

#[test]
fn test_serialize_newtype_struct_with_string() {
    struct TestString<'a>(&'a str);

    impl Serialize for TestString<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.0)
        }
    }

    let serializer = MapKeySerializer;
    let value = TestString("test");
    let result = serializer.serialize_newtype_struct("test_name", &value).unwrap();
    assert_eq!(result, "test");
}

#[test]
fn test_serialize_newtype_struct_with_integer() {
    struct TestInteger(i32);

    impl Serialize for TestInteger {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok>
        where
            S: serde::Serializer,
        {
            serializer.serialize_i32(self.0)
        }
    }

    let serializer = MapKeySerializer;
    let value = TestInteger(42);
    let result = serializer.serialize_newtype_struct("integer_name", &value).unwrap();
    assert_eq!(result, "42");
}

#[should_panic]
fn test_serialize_newtype_struct_should_panic() {
    // Here we use a type that doesn't implement Serialize
    struct NonSerializable;

    let serializer = MapKeySerializer;
    let value = NonSerializable;
    let _result = serializer.serialize_newtype_struct("non_serializable", &value).unwrap();
}

