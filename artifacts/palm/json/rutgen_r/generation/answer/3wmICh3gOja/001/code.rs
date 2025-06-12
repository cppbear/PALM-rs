// Answer 0

#[test]
fn test_end_with_valid_sequence() {
    struct TestSerializer;

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = serde::ser::Error;

        // required methods for the Serializer trait
        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
            Ok(TestSerializeSeq { is_ended: false })
        }

        // Other required methods omitted for brevity
    }

    struct TestSerializeSeq {
        is_ended: bool,
    }

    impl serde::ser::SerializeSeq for TestSerializeSeq {
        type Ok = ();
        type Error = serde::ser::Error;

        fn end(self) -> Result<Self::Ok> {
            if self.is_ended {
                Err(serde::ser::Error::custom("sequence already ended"))
            } else {
                Ok(())
            }
        }

        // Other required methods omitted for brevity
    }

    let serializer = TestSerializer;
    let seq = serializer.serialize_seq(Some(1)).unwrap();
    let result = seq.end();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "sequence already ended")]
fn test_end_with_panic_on_double_end() {
    struct TestSerializer;

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = serde::ser::Error;

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
            Ok(TestSerializeSeq { is_ended: false })
        }
    }

    struct TestSerializeSeq {
        is_ended: bool,
    }

    impl serde::ser::SerializeSeq for TestSerializeSeq {
        type Ok = ();
        type Error = serde::ser::Error;

        fn end(self) -> Result<Self::Ok> {
            if self.is_ended {
                panic!("sequence already ended");
            } else {
                Ok(())
            }
        }
    }

    let serializer = TestSerializer;
    let seq = serializer.serialize_seq(Some(1)).unwrap();
    let _ = seq.end(); // First call should not panic
    let _ = seq.end(); // Second call should panic
}

#[test]
fn test_end_with_empty_sequence() {
    struct TestSerializer;

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = serde::ser::Error;

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
            Ok(TestSerializeSeq { is_ended: false })
        }
    }

    struct TestSerializeSeq {
        is_ended: bool,
    }

    impl serde::ser::SerializeSeq for TestSerializeSeq {
        type Ok = ();
        type Error = serde::ser::Error;

        fn end(self) -> Result<Self::Ok> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let seq = serializer.serialize_seq(Some(0)).unwrap();
    let result = seq.end();
    assert!(result.is_ok());
}

