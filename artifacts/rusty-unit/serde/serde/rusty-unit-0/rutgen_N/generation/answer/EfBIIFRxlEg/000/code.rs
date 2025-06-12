// Answer 0

#[derive(Deserialize)]
struct NewTypeStruct {
    value: i32,
}

struct TestVisitor;

impl<'de> Visitor<'de> for TestVisitor {
    type Value = NewTypeStruct;

    fn visit_newtype_struct<E>(self, _: E) -> Result<NewTypeStruct, E::Error>
    where
        E: de::Decoder<'de>,
    {
        Ok(NewTypeStruct { value: 42 })
    }
}

#[test]
fn test_deserialize_newtype_struct() {
    let visitor = TestVisitor;
    let result = deserialize_newtype_struct("NewTypeStruct", visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 42);
}

