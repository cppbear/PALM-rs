// Answer 0

#[derive(Debug)]
struct DummyError;

impl de::Error for DummyError {}

struct DummyVisitor<'de> {
    name: &'de str,
}

impl<'de> DummyVisitor<'de> {
    fn visit_borrowed_str<F>(self, value: &'de str) -> Result<TagOrContent, F>
    where
        F: de::Error,
    {
        if value == self.name {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new()
                .visit_borrowed_str(value)
                .map(TagOrContent::Content)
        }
    }
}

struct ContentVisitor;

impl ContentVisitor {
    fn new() -> Self {
        ContentVisitor
    }

    fn visit_borrowed_str<F>(self, value: &str) -> Result<&str, F>
    where
        F: de::Error,
    {
        Ok(value) // Simulating successful result with borrowed string reference
    }
}

enum TagOrContent {
    Tag,
    Content(&'static str),
}

#[test]
fn test_visit_borrowed_str_content() {
    let visitor = DummyVisitor { name: "expected" };
    let input = "actual";

    let result: Result<TagOrContent, DummyError> = visitor.visit_borrowed_str(input);

    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            TagOrContent::Content(val) => assert_eq!(val, input),
            _ => panic!("Expected TagOrContent::Content variant"),
        }
    }
}

#[test]
fn test_visit_borrowed_str_tag() {
    let visitor = DummyVisitor { name: "expected" };
    let input = "expected";

    let result: Result<TagOrContent, DummyError> = visitor.visit_borrowed_str(input);

    assert!(result.is_ok());
    if let Ok(tag) = result {
        assert_eq!(tag, TagOrContent::Tag);
    }
}

