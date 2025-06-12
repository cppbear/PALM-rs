// Answer 0

#[derive(Debug)]
struct MyVisitor(Option<String>);

impl MyVisitor {
    fn visit_string<E>(self, v: String) -> Result<Option<String>, E>
    where
        E: std::fmt::Debug,
    {
        *self.0 = Some(v);
        Ok(self.0)
    }
}

#[test]
fn test_visit_string() {
    let mut visitor = MyVisitor(None);
    let result: Result<Option<String>, _> = visitor.visit_string("test".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("test".to_string()));
}

#[test]
fn test_visit_string_empty() {
    let mut visitor = MyVisitor(None);
    let result: Result<Option<String>, _> = visitor.visit_string("".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("".to_string()));
}

