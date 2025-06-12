// Answer 0

#[test]
fn test_serialize_string() {
    use serde::ser::Serializer;
    use serde::ser::Ser;

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = String;
        type Error = ();
        
        fn collect_str<T>(&mut self, value: T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + std::fmt::Display,
        {
            Ok(value.to_string())
        }
    }

    let serializer = TestSerializer;
    let test_value = "Test String";
    let result = serialize(&test_value, serializer);
    assert_eq!(result, Ok("Test String".to_string()));
}

#[test]
fn test_serialize_empty_string() {
    use serde::ser::Serializer;
    use serde::ser::Ser;

    struct TestSerializer;

    impl Serializer for TestSerializer {
        type Ok = String;
        type Error = ();
        
        fn collect_str<T>(&mut self, value: T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + std::fmt::Display,
        {
            Ok(value.to_string())
        }
    }

    let serializer = TestSerializer;
    let test_value = "";
    let result = serialize(&test_value, serializer);
    assert_eq!(result, Ok("".to_string()));
}

#[test]
#[should_panic]
fn test_serialize_panic() {
    use serde::ser::Serializer;
    
    struct PanickingSerializer;

    impl Serializer for PanickingSerializer {
        type Ok = String;
        type Error = ();

        fn collect_str<T>(&mut self, _value: T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + std::fmt::Display,
        {
            panic!("Intentional Panic");
        }
    }

    let serializer = PanickingSerializer;
    let test_value = "Should Panic";
    serialize(&test_value, serializer);
}

