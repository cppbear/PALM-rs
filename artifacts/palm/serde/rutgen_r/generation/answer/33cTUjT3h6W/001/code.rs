// Answer 0

#[test]
fn test_serialize_char_returns_error() {
    struct SerdeSerializer;

    impl SerdeSerializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported type".to_string()
        }

        fn serialize_char(self, _: char) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Char))
        }
    }

    struct Unsupported;

    impl Unsupported {
        const Char: Self = Unsupported;
    }

    let serializer = SerdeSerializer;
    let result = serializer.serialize_char('a');
    assert_eq!(result, Err("Unsupported type".to_string()));
}

#[test]
fn test_serialize_char_with_different_chars() {
    struct SerdeSerializer;

    impl SerdeSerializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported type".to_string()
        }

        fn serialize_char(self, _: char) -> Result<(), String> {
            Err(Self::bad_type(Unsupported::Char))
        }
    }

    struct Unsupported;

    impl Unsupported {
        const Char: Self = Unsupported;
    }

    let serializer = SerdeSerializer;

    let special_char_result = serializer.serialize_char('#');
    assert_eq!(special_char_result, Err("Unsupported type".to_string()));

    let another_char_result = serializer.serialize_char('Z');
    assert_eq!(another_char_result, Err("Unsupported type".to_string()));
}

