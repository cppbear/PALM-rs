// Answer 0

#[derive(Debug)]
struct TaggedContentVisitor<'a> {
    tag_name: &'a str,
    expecting: &'a str,
    value: std::marker::PhantomData<()>,
}

impl<'a> TaggedContentVisitor<'a> {
    pub fn new(name: &'static str, expecting: &'static str) -> Self {
        TaggedContentVisitor {
            tag_name: name,
            expecting,
            value: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_new_with_non_empty_strings() {
    let visitor = TaggedContentVisitor::new("tag_name", "expecting_value");
    assert_eq!(visitor.tag_name, "tag_name");
    assert_eq!(visitor.expecting, "expecting_value");
}

#[test]
fn test_new_with_empty_strings() {
    let visitor = TaggedContentVisitor::new("", "");
    assert_eq!(visitor.tag_name, "");
    assert_eq!(visitor.expecting, "");
}

#[test]
fn test_new_with_large_strings() {
    let large_name = "a".repeat(1000);
    let large_expecting = "b".repeat(1000);
    let visitor = TaggedContentVisitor::new(&large_name, &large_expecting);
    assert_eq!(visitor.tag_name, large_name);
    assert_eq!(visitor.expecting, large_expecting);
}

#[test]
#[should_panic]
fn test_new_with_null_string() {
    let visitor = TaggedContentVisitor::new(std::ptr::null_mut(), "valid_expecting");
}

#[test]
#[should_panic]
fn test_new_with_null_expecting() {
    let visitor = TaggedContentVisitor::new("valid_name", std::ptr::null_mut());
}

