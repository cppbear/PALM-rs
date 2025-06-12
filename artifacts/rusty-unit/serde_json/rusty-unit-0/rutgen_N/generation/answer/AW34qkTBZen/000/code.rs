// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct Serializer;

    impl Serializer {
        fn serialize_bool(self, value: bool) -> Result<String, &'static str> {
            Ok(if value { "true" } else { "false" }.to_owned())
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_bool(true).unwrap();
    assert_eq!(result, "true");
}

#[test]
fn test_serialize_bool_false() {
    struct Serializer;

    impl Serializer {
        fn serialize_bool(self, value: bool) -> Result<String, &'static str> {
            Ok(if value { "true" } else { "false" }.to_owned())
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_bool(false).unwrap();
    assert_eq!(result, "false");
}

