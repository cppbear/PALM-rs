// Answer 0

#[test]
fn test_tagged_content_visitor_new() {
    use std::marker::PhantomData;

    struct TaggedContentVisitor<T> {
        tag_name: &'static str,
        expecting: &'static str,
        value: PhantomData<T>,
    }

    fn new<T>(name: &'static str, expecting: &'static str) -> TaggedContentVisitor<T> {
        TaggedContentVisitor {
            tag_name: name,
            expecting,
            value: PhantomData,
        }
    }

    // Test with valid string inputs
    let visitor1: TaggedContentVisitor<()> = new("tag1", "Expecting type 1");
    assert_eq!(visitor1.tag_name, "tag1");
    assert_eq!(visitor1.expecting, "Expecting type 1");

    let visitor2: TaggedContentVisitor<()> = new("tag2", "Expecting type 2");
    assert_eq!(visitor2.tag_name, "tag2");
    assert_eq!(visitor2.expecting, "Expecting type 2");

    // Test with different valid string inputs
    let visitor3: TaggedContentVisitor<()> = new("tag3", "Expecting type 3");
    assert_eq!(visitor3.tag_name, "tag3");
    assert_eq!(visitor3.expecting, "Expecting type 3");
}

#[test]
#[should_panic]
fn test_tagged_content_visitor_new_empty_name() {
    use std::marker::PhantomData;

    struct TaggedContentVisitor<T> {
        tag_name: &'static str,
        expecting: &'static str,
        value: PhantomData<T>,
    }

    fn new<T>(name: &'static str, expecting: &'static str) -> TaggedContentVisitor<T> {
        TaggedContentVisitor {
            tag_name: name,
            expecting,
            value: PhantomData,
        }
    }

    // This test assumes that an empty name would cause an error/panic
    let _visitor: TaggedContentVisitor<()> = new("", "Expecting type 1");
}

#[test]
#[should_panic]
fn test_tagged_content_visitor_new_empty_expecting() {
    use std::marker::PhantomData;

    struct TaggedContentVisitor<T> {
        tag_name: &'static str,
        expecting: &'static str,
        value: PhantomData<T>,
    }

    fn new<T>(name: &'static str, expecting: &'static str) -> TaggedContentVisitor<T> {
        TaggedContentVisitor {
            tag_name: name,
            expecting,
            value: PhantomData,
        }
    }

    // This test assumes that an empty expecting would cause an error/panic
    let _visitor: TaggedContentVisitor<()> = new("tag1", "");
}

