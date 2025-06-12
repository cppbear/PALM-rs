// Answer 0

#[derive(Debug)]
struct EnumAccessDeserializer<A> {
    access: A,
}

impl<A> EnumAccessDeserializer<A> {
    pub fn new(access: A) -> Self {
        EnumAccessDeserializer { access }
    }
}

#[test]
fn test_enum_access_deserializer_new() {
    struct TestAccess;

    let access = TestAccess;
    let deserializer = EnumAccessDeserializer::new(access);
    assert_eq!(std::mem::size_of_val(&deserializer.access), std::mem::size_of::<TestAccess>());
}

#[test]
fn test_enum_access_deserializer_new_with_different_access() {
    struct AnotherAccess;

    let access = AnotherAccess;
    let deserializer = EnumAccessDeserializer::new(access);
    assert_eq!(std::mem::size_of_val(&deserializer.access), std::mem::size_of::<AnotherAccess>());
}

