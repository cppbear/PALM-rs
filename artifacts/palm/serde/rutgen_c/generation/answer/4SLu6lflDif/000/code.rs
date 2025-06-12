// Answer 0

#[test]
fn test_enum_access_deserializer_new() {
    struct MockAccess;

    let access = MockAccess;
    let enum_deserializer = EnumAccessDeserializer::new(access);

    // Optionally check the state of the enum_deserializer if necessary
    assert_eq!(std::mem::size_of_val(&enum_deserializer), std::mem::size_of::<EnumAccessDeserializer<MockAccess>>());
}

