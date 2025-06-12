// Answer 0

#[test]
fn test_serialize_value() {
    struct MockSerializer;

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), &'static str>
        where
            T: ?Sized + serde::ser::Serialize,
        {
            let _ = value;
            Err("impossible")
        }
    }

    struct TestStruct {
        field: i32,
    }

    impl serde::ser::Serialize for TestStruct {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: serde::Serializer,
        {
            Ok(())
        }
    }

    let mut serializer = MockSerializer::new();
    let value = TestStruct { field: 42 };

    let result = serializer.serialize_value(&value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "impossible");
}

#[test]
fn test_serialize_value_empty_struct() {
    struct MockSerializer;

    impl MockSerializer {
        fn new() -> Self {
            MockSerializer
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), &'static str>
        where
            T: ?Sized + serde::ser::Serialize,
        {
            let _ = value;
            Err("impossible")
        }
    }

    struct EmptyStruct;

    impl serde::ser::Serialize for EmptyStruct {
        fn serialize<S>(&self, _: S) -> Result<(), S::Error>
        where
            S: serde::Serializer,
        {
            Ok(())
        }
    }

    let mut serializer = MockSerializer::new();
    let value = EmptyStruct;

    let result = serializer.serialize_value(&value);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "impossible");
}

