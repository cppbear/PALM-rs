// Answer 0

#[test]
fn test_deserialize_i64_negative() {
    let content = Content::I64(-9223372036854775808);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_i64(visitor);
}

#[test]
fn test_deserialize_i64_negative_one() {
    let content = Content::I64(-1);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_i64(visitor);
}

#[test]
fn test_deserialize_i64_zero() {
    let content = Content::I64(0);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_i64(visitor);
}

#[test]
fn test_deserialize_i64_positive_one() {
    let content = Content::I64(1);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_i64(visitor);
}

#[test]
fn test_deserialize_i64_positive_max() {
    let content = Content::I64(9223372036854775807);
    let deserializer = ContentDeserializer { content, err: PhantomData };
    let visitor = TestVisitor;
    let _ = deserializer.deserialize_i64(visitor);
}

