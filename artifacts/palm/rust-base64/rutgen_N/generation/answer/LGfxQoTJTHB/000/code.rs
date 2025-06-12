// Answer 0

#[test]
fn test_flush_success() {
    struct TestWriter;

    impl TestWriter {
        fn flush(&mut self) -> std::io::Result<()> {
            // no op
            Ok(())
        }
    }

    let mut writer = TestWriter;
    let result = writer.flush();
    assert!(result.is_ok());
}

