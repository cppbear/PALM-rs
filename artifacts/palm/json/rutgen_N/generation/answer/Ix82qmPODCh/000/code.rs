// Answer 0

#[test]
fn test_serialize_seq_end_success() {
    struct TestSerializeSeq {
        finished: bool,
    }

    impl TestSerializeSeq {
        fn end(self) -> Result<()> {
            if self.finished {
                Ok(())
            } else {
                Err("Sequence not finished".into())
            }
        }
    }

    let seq = TestSerializeSeq { finished: true };
    let result = seq.end();
    assert!(result.is_ok());
}

#[test]
fn test_serialize_seq_end_failure() {
    struct TestSerializeSeq {
        finished: bool,
    }

    impl TestSerializeSeq {
        fn end(self) -> Result<()> {
            if self.finished {
                Ok(())
            } else {
                Err("Sequence not finished".into())
            }
        }
    }

    let seq = TestSerializeSeq { finished: false };
    let result = seq.end();
    assert!(result.is_err());
}

