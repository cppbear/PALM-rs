// Answer 0

#[test]
fn test_serialize_i16() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> &'static str {
            "bad type"
        }
    }

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = &'static str;

        fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
            Err(SelfSerializer::bad_type(Unsupported::Integer))
        }

        fn is_human_readable(&self) -> bool {
            false
        }

        // Other required methods omitted for brevity
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i16(42);
    assert_eq!(result, Err("bad type"));
}

