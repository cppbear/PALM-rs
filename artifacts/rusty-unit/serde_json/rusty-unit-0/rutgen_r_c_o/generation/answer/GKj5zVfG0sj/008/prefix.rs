// Answer 0

#[test]
fn test_deserialize_struct_valid_object() {
    let input: &[u8] = b"{\"key\":\"value\"}";
    let mut deserializer = Deserializer::new(input);
    deserializer.remaining_depth = 0;
    let visitor = MyVisitor;
    let _ = deserializer.deserialize_struct("MyObject", &["key"], visitor);
}

#[test]
fn test_deserialize_struct_invalid_whitespace() {
    let input: &[u8] = b"   ";
    let mut deserializer = Deserializer::new(input);
    let visitor = MyVisitor;
    let result = deserializer.deserialize_struct("MyObject", &["key"], visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_struct_error_end_map() {
    let input: &[u8] = b"{\"key\":\"value\",}";
    let mut deserializer = Deserializer::new(input);
    deserializer.remaining_depth = 1;
    let visitor = MyVisitor;
    let result = deserializer.deserialize_struct("MyObject", &["key"], visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_struct_empty_object() {
    let input: &[u8] = b"{}";
    let mut deserializer = Deserializer::new(input);
    deserializer.remaining_depth = 0;
    let visitor = MyVisitor;
    let _ = deserializer.deserialize_struct("EmptyObject", &[], visitor);
}

#[test]
fn test_deserialize_struct_nested_object() {
    let input: &[u8] = b"{\"outer\":{\"inner\":\"value\"}}";
    let mut deserializer = Deserializer::new(input);
    deserializer.remaining_depth = 2; // Allow nesting
    let visitor = MyVisitor;
    let _ = deserializer.deserialize_struct("NestedObject", &["outer"], visitor);
}

#[test]
#[should_panic]
fn test_deserialize_struct_exceed_recursion_limit() {
    let input: &[u8] = b"{\"key\":{\"key\":{\"key\":{\"key\":{\"key\":\"value\"}}}}}";
    let mut deserializer = Deserializer::new(input);
    deserializer.remaining_depth = 0; // Set to zero to exceed limit
    let visitor = MyVisitor;
    let _ = deserializer.deserialize_struct("DeepNestedObject", &["key"], visitor);
}

struct MyVisitor;

impl<'de> de::Visitor<'de> for MyVisitor {
    type Value = ();
    
    fn visit_seq<S>(self, _seq: S) -> Result<Self::Value>
    where
        S: de::SeqAccess<'de>,
    {
        Ok(())
    }

    fn visit_map<M>(self, _map: M) -> Result<Self::Value>
    where
        M: de::MapAccess<'de>,
    {
        Ok(())
    }
}

