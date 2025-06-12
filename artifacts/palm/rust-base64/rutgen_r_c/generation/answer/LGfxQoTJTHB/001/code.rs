// Answer 0

#[test]
fn test_flush_no_op() {
    struct DummyStrConsumer;

    impl StrConsumer for DummyStrConsumer {
        fn consume(&mut self, _buf: &str) {
            // no op
        }
    }

    let consumer = DummyStrConsumer;
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };

    let result = writer.flush();
    assert_eq!(result, Ok(()));
}

