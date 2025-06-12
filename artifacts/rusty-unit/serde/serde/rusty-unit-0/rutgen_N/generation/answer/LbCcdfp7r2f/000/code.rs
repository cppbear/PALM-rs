// Answer 0

#[derive(Clone)]
struct TestContent<'de> {
    data: &'de str,
}

struct TestVisitor {
    value: String,
}

impl<'de> serde::de::Visitor<'de> for TestVisitor {
    type Value = String;

    fn visit_seq<V>(&mut self, _: &mut V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        self.value.push_str("Visited sequence.");
        Ok(self.value.clone())
    }
}

#[test]
fn test_visit_content_seq_ref() {
    let content = vec![TestContent { data: "a" }, TestContent { data: "b" }];
    let visitor = TestVisitor { value: String::new() };
    let result: Result<String, serde::de::value::Error> = visit_content_seq_ref(&content, visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Visited sequence.");
}

