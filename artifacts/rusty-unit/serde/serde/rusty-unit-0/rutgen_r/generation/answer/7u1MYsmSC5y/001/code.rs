// Answer 0

#[derive(Debug)]
struct TagOrContentVisitor<'a> {
    name: &'a str,
    value: std::marker::PhantomData<()>,
}

impl<'a> TagOrContentVisitor<'a> {
    fn new(name: &'static str) -> Self {
        TagOrContentVisitor {
            name,
            value: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_tag_or_content_visitor_new_valid_name() {
    let name: &'static str = "valid_name";
    let visitor = TagOrContentVisitor::new(name);
    assert_eq!(visitor.name, name);
}

#[test]
fn test_tag_or_content_visitor_new_empty_name() {
    let name: &'static str = "";
    let visitor = TagOrContentVisitor::new(name);
    assert_eq!(visitor.name, name);
}

#[test]
fn test_tag_or_content_visitor_new_long_name() {
    let name: &'static str = "a_long_name_with_more_than_20_characters";
    let visitor = TagOrContentVisitor::new(name);
    assert_eq!(visitor.name, name);
}

