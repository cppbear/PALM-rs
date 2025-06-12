// Answer 0

#[test]
fn test_serialize_seq_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> String {
            "bad type".to_string()
        }
    }

    impl Serializer for TestSerializer {
        type SerializeSeq = ();
        type Error = String;

        fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            Err(Self::bad_type(Unsupported::Sequence))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_seq(None);
    assert_eq!(result.err(), Some("bad type".to_string()));
}

