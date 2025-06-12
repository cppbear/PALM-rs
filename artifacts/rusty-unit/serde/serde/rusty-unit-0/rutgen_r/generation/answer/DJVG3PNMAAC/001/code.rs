// Answer 0

#[test]
fn test_serialize_unit_struct_valid() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit_struct(self, name: &'static str) -> Result<Content, &'static str> {
            Ok(Content::UnitStruct(name))
        }
    }

    enum Content {
        UnitStruct(&'static str),
    }

    let serializer = TestSerializer;
    let name = "TestStruct";

    let result = serializer.serialize_unit_struct(name);
    assert!(result.is_ok());

    if let Ok(content) = result {
        match content {
            Content::UnitStruct(n) => assert_eq!(n, name),
        }
    }
}

#[test]
#[should_panic]
fn test_serialize_unit_struct_empty_name() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit_struct(self, name: &'static str) -> Result<Content, &'static str> {
            Ok(Content::UnitStruct(name))
        }
    }

    enum Content {
        UnitStruct(&'static str),
    }

    let serializer = TestSerializer;
    let name = ""; // An empty name could be a conceptual boundary condition.

    let result = serializer.serialize_unit_struct(name);
    assert!(result.is_ok()); // This should not panic, for completeness.
}

