// Answer 0

#[derive(Default)]
struct ContentVisitor;

impl ContentVisitor {
    fn new() -> Self {
        ContentVisitor::default()
    }

    fn visit_bool(self, value: bool) -> Result<bool, serde::de::Error> {
        Ok(value)
    }
}

struct TagOrContent<T> {
    content: T,
}

impl TagOrContent<bool> {
    fn Content(content: bool) -> Self {
        TagOrContent { content }
    }
}

#[derive(Default)]
struct De {}

impl De {
    fn visit_bool<F>(self, value: bool) -> Result<TagOrContent<bool>, F>
    where
        F: serde::de::Error,
    {
        ContentVisitor::new()
            .visit_bool(value)
            .map(TagOrContent::Content)
    }
}

#[test]
fn test_visit_bool_true() {
    let de = De::default();
    let result: Result<TagOrContent<bool>, serde::de::Error> = de.visit_bool(true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, true);
}

#[test]
fn test_visit_bool_false() {
    let de = De::default();
    let result: Result<TagOrContent<bool>, serde::de::Error> = de.visit_bool(false);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().content, false);
}

