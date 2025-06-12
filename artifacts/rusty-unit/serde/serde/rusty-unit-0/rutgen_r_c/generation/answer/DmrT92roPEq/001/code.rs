// Answer 0

#[test]
fn test_serialize_unit() {
    struct TestSerializer;

    impl std::fmt::Write for TestSerializer {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let serializer = &mut TestSerializer;
    let result = serializer.serialize_unit();
    assert!(result.is_err());
}

#[test]
fn test_serialize_unit_error() {
    struct TestSerializer;

    impl std::fmt::Write for TestSerializer {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Ok(())
        }
    }

    let serializer = &mut TestSerializer;
    let result = serializer.serialize_unit();
    assert_eq!(result, Err(std::fmt::Error));
}

