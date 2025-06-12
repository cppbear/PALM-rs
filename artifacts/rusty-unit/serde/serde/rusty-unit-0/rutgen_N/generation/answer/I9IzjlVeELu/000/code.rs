// Answer 0

#[derive(Debug)]
struct ContentVisitor;

impl ContentVisitor {
    fn new() -> Self {
        ContentVisitor
    }

    fn visit_str(self, value: &str) -> Result<&'static str, ()> {
        Ok(value)
    }
}

#[derive(Debug)]
struct MyError;

impl de::Error for MyError {}

#[derive(Debug)]
enum TagOrContent {
    Tag,
    Content(&'static str),
}

#[derive(Debug)]
struct MyVisitor {
    name: &'static str,
}

impl MyVisitor {
    fn visit_str<F>(self, value: &str) -> Result<TagOrContent, F>
    where
        F: de::Error,
    {
        if value == self.name {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new()
                .visit_str(value)
                .map(TagOrContent::Content)
        }
    }
}

#[test]
fn test_visit_str_tag() {
    let visitor = MyVisitor { name: "tag" };
    assert_eq!(visitor.visit_str("tag"), Ok(TagOrContent::Tag));
}

#[test]
fn test_visit_str_content() {
    let visitor = MyVisitor { name: "tag" };
    assert_eq!(visitor.visit_str("content"), Ok(TagOrContent::Content("content")));
}

