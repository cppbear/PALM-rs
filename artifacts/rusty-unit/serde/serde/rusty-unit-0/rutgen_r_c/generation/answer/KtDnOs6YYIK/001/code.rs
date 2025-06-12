// Answer 0

#[test]
fn test_visit_u64_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "Expecting a field index for TagContentOtherField")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_u64(0);
    assert_eq!(result, Ok(TagContentOtherField::Tag));
}

#[test]
fn test_visit_u64_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "Expecting a field index for TagContentOtherField")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_u64(1);
    assert_eq!(result, Ok(TagContentOtherField::Content));
}

#[test]
fn test_visit_u64_other() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagContentOtherField;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "Expecting a field index for TagContentOtherField")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_u64(2);
    assert_eq!(result, Ok(TagContentOtherField::Other));
}

