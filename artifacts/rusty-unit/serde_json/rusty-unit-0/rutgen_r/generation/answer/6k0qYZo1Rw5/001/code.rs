// Answer 0

#[test]
fn test_serialize_seq_end_success() {
    struct TestSerializeSeq;

    impl ser::SerializeSeq for TestSerializeSeq {
        fn end(self) -> Result<()> {
            Ok(())
        }
    }

    let serializer = TestSerializeSeq;
    let result = serializer.end();
    assert!(result.is_ok());
}

#[test]
fn test_serialize_seq_end_failure() {
    struct TestSerializeSeq;

    impl ser::SerializeSeq for TestSerializeSeq {
        fn end(self) -> Result<()> {
            Err(Error::custom("serialization error"))
        }
    }

    let serializer = TestSerializeSeq;
    let result = serializer.end();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "serialization error");
}

