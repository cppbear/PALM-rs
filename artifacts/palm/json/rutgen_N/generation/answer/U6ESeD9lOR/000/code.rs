// Answer 0

#[derive(Debug)]
struct TestVisitor {
    value: Option<i32>,
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = Option<i32>;

    fn visit_some<D>(self, _: D) -> Result<Self::Value, serde::de::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(self.value)
    }

    fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
        Ok(None)
    }
}

#[test]
fn test_deserialize_option_some() {
    let value = Some(42);
    let visitor = TestVisitor { value };
    let result: Result<Option<i32>, serde::de::Error> = visitor.visit_some(());
    assert_eq!(result.unwrap(), Some(42));
}

#[test]
fn test_deserialize_option_none() {
    let visitor = TestVisitor { value: None };
    let result: Result<Option<i32>, serde::de::Error> = visitor.visit_none();
    assert_eq!(result.unwrap(), None);
}

