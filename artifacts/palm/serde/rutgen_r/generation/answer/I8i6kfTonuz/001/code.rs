// Answer 0

#[test]
fn test_serialize_bool_err() {
    struct MockSerializer;

    impl MockSerializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported type".to_string()
        }
    }

    trait Serializer {
        type Ok;
        type Error;

        fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error>;
    }

    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = String;

        fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
            Err(Self::bad_type(Unsupported::Boolean))
        }
    }

    enum Unsupported {
        Boolean,
    }

    let serializer = MockSerializer;

    let result: Result<(), String> = serializer.serialize_bool(true);
    assert_eq!(result, Err("Unsupported type".to_string()));

    let result_false: Result<(), String> = serializer.serialize_bool(false);
    assert_eq!(result_false, Err("Unsupported type".to_string()));
}

