// Answer 0

#[test]
fn test_flush_no_op() {
    struct TestStrConsumer;

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, _: &str) {
            // No-op
        }
    }

    let mut writer = Utf8SingleCodeUnitWriter {
        str_consumer: TestStrConsumer,
    };

    let result = writer.flush();
    assert!(result.is_ok());
}

