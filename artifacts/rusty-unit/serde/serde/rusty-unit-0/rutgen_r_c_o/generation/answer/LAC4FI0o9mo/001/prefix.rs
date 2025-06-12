// Answer 0

#[test]
fn test_visit_borrowed_bytes_with_matching_name() {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            writeln!(formatter, "Expecting bytes matching name")
        }
    }

    let visitor = TestVisitor { name: "tag_name" };
    let value: &[u8] = b"tag_name";
    let _ = visitor.visit_borrowed_bytes(value);
}

#[test]
fn test_visit_borrowed_bytes_with_different_value() {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            writeln!(formatter, "Expecting bytes matching name")
        }
    }

    let visitor = TestVisitor { name: "tag_name" };
    let value: &[u8] = b"different_value";
    let _ = visitor.visit_borrowed_bytes(value);
}

