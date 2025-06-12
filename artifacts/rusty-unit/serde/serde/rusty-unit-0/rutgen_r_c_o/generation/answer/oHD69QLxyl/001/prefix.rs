// Answer 0

#[test]
fn test_end_with_simple_struct() {
    let name = "test_struct";
    let fields = vec![Content::U8(123)];
    let serializer = SerializeTupleStruct { name, fields, error: PhantomData };
    serializer.end();
}

#[test]
fn test_end_with_multiple_fields() {
    let name = "multi_field_struct";
    let fields = vec![
        Content::Bool(true),
        Content::F32(3.14),
        Content::String("example".to_string()),
    ];
    let serializer = SerializeTupleStruct { name, fields, error: PhantomData };
    serializer.end();
}

#[test]
fn test_end_with_various_content_types() {
    let name = "various_content";
    let fields = vec![
        Content::I32(-42),
        Content::Char('c'),
        Content::Unit,
        Content::Some(Box::new(Content::Str("some_str"))),
        Content::Newtype(Box::new(Content::U64(100))),
    ];
    let serializer = SerializeTupleStruct { name, fields, error: PhantomData };
    serializer.end();
}

#[test]
fn test_end_with_maximum_fields() {
    let name = "max_fields_struct";
    let fields = (0..100).map(|i| Content::U32(i)).collect();
    let serializer = SerializeTupleStruct { name, fields, error: PhantomData };
    serializer.end();
}

#[test]
fn test_end_with_long_name_and_fields() {
    let name = "long_name_struct_with_a_reasonable_length";
    let fields = vec![
        Content::Bytes(vec![1, 2, 3]),
        Content::F64(2.718),
        Content::UnitStruct("unit_struct_name"),
    ];
    let serializer = SerializeTupleStruct { name, fields, error: PhantomData };
    serializer.end();
}

