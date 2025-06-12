// Answer 0

#[test]
fn test_deserialize_bytes_str_a() {
    let content = Content::Str("a");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor {};
    deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_str_b() {
    let content = Content::Str("b");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor {};
    deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_str_c() {
    let content = Content::Str("c");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor {};
    deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_str_z() {
    let content = Content::Str("z");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor {};
    deserializer.deserialize_bytes(visitor);
}

#[test]
fn test_deserialize_bytes_str_empty() {
    let content = Content::Str("");
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor {};
    deserializer.deserialize_bytes(visitor);
}

struct TestVisitor;

impl<'de> Visitor<'de> for TestVisitor {
    type Value = ();

    fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_borrowed_bytes<E>(self, _: &'de [u8]) -> Result<Self::Value, E> {
        Ok(())
    }

    fn visit_seq<V>(self, _: V) -> Result<Self::Value, E> where V: SeqAccess<'de> {
        Ok(())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(())
    }
}

