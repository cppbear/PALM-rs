// Answer 0

#[test]
fn test_serialize_unit() {
    struct TestSerializer;

    impl std::fmt::Write for TestSerializer {
        fn write_str(&mut self, _s: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let mut serializer = TestSerializer;

    let result = serializer.serialize_unit();
    assert!(result.is_err());
}

#[test]
fn test_serialize_unit_empty() {
    struct EmptyTestSerializer;

    impl std::fmt::Write for EmptyTestSerializer {
        fn write_str(&mut self, _s: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let mut serializer = EmptyTestSerializer;

    let result = serializer.serialize_unit();
    assert!(result.is_err());
}

