// Answer 0

#[test]
fn test_deserialize_unit_struct_bool() {
    let content = Content::Bool(true);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    deserializer.deserialize_unit_struct("Test", visitor);
}

#[test]
fn test_deserialize_unit_struct_string() {
    let content = Content::String(String::from("data"));
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    deserializer.deserialize_unit_struct("Test", visitor);
}

#[test]
fn test_deserialize_unit_struct_char() {
    let content = Content::Char('a');
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    deserializer.deserialize_unit_struct("Test", visitor);
}

#[test]
fn test_deserialize_unit_struct_i32() {
    let content = Content::I32(42);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    deserializer.deserialize_unit_struct("Test", visitor);
}

#[test]
fn test_deserialize_unit_struct_u64() {
    let content = Content::U64(100);
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    deserializer.deserialize_unit_struct("Test", visitor);
}

#[test]
fn test_deserialize_unit_struct_newtype() {
    let content = Content::Newtype(Box::new(Content::U8(1)));
    let deserializer = ContentDeserializer {
        content,
        err: PhantomData,
    };
    let visitor = MyVisitor {};
    deserializer.deserialize_unit_struct("Test", visitor);
}

