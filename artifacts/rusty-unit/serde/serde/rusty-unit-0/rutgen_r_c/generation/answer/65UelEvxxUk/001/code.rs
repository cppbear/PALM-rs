// Answer 0

#[test]
fn test_visit_u32_with_value_zero() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "expecting u32 value")
        }
    }

    let visitor = TestVisitor {};
    let result = visitor.visit_u32(0u32);
    assert!(matches!(result, Ok(TagOrContent::Content(Content::U32(0)))));
}

#[test]
fn test_visit_u32_with_value_max() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "expecting u32 value")
        }
    }

    let visitor = TestVisitor {};
    let result = visitor.visit_u32(u32::MAX);
    assert!(matches!(result, Ok(TagOrContent::Content(Content::U32(u32::MAX)))));
}

#[test]
fn test_visit_u32_with_value_mid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            write!(fmt, "expecting u32 value")
        }
    }

    let visitor = TestVisitor {};
    let result = visitor.visit_u32(12345u32);
    assert!(matches!(result, Ok(TagOrContent::Content(Content::U32(12345)))));
}

