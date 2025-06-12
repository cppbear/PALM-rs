// Answer 0

#[test]
fn test_serialize_valid_char() {
    let value: char = 'a';
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }
    let _ = value.serialize(MockSerializer);
}

#[test]
fn test_serialize_invalid_char() {
    let value: char = '\u{10FFFF}';
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }
    let _ = value.serialize(MockSerializer);
}

#[test]
fn test_serialize_empty_string() {
    let value: &str = "";
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }
    let _ = value.serialize(MockSerializer);
}

#[test]
fn test_serialize_long_string() {
    let value: &str = "a".repeat(65536);
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }
    let _ = value.serialize(MockSerializer);
}

#[should_panic]
#[test]
fn test_serialize_control_character() {
    let value: char = '\u{0000}';
    struct MockSerializer;
    impl Serializer for MockSerializer {
        type Ok = ();
        type Error = ();
        fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }
    let _ = value.serialize(MockSerializer);
}

