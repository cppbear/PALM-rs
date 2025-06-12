// Answer 0

#[test]
fn test_serialize_unit() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit(self) -> Result<Content, &'static str> {
            Ok(Content::Unit)
        }
    }

    struct Content {
        unit: bool,
    }

    impl Content {
        const Unit: Content = Content { unit: true };
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_unit();
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Content::Unit);
}

