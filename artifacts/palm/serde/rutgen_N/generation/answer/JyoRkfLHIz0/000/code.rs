// Answer 0

#[derive(Debug)]
struct TestVisitor;

impl<'de> Visitor<'de> for TestVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }
}

#[test]
fn test_deserialize_ignored_any() {
    let visitor = TestVisitor;
    let result: Result<(), serde::de::Error> = deserialize_ignored_any(visitor);
    assert!(result.is_ok());
}

