// Answer 0

#[test]
fn test_content_visitor_new() {
    struct TestVisitor<'de> {
        _phantom: PhantomData<&'de ()>,
    }

    let visitor = ContentVisitor::new();
    assert_eq!(std::mem::size_of_val(&visitor), std::mem::size_of::<ContentVisitor>());
}

