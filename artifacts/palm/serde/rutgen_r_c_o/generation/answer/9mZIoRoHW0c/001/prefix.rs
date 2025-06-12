// Answer 0

#[test]
fn test_deserialize_u32_valid_min() {
    let content = Content::U32(0);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    deserializer.deserialize_u32(visitor);
}

#[test]
fn test_deserialize_u32_valid_max() {
    let content = Content::U32(4294967295);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    deserializer.deserialize_u32(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_u32_invalid_negative() {
    let content = Content::I32(-1);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    deserializer.deserialize_u32(visitor);
}

#[test]
#[should_panic]
fn test_deserialize_u32_invalid_out_of_range() {
    let content = Content::U32(4294967296);
    let deserializer = ContentRefDeserializer { content: &content, err: PhantomData };
    deserializer.deserialize_u32(visitor);
}

