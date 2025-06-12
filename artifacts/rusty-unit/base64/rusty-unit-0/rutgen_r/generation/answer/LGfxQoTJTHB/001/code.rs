// Answer 0

#[test]
fn test_flush_success() {
    struct EncoderStringWriter;

    impl EncoderStringWriter {
        fn flush(&mut self) -> std::io::Result<()> {
            // no op
            Ok(())
        }
    }

    let mut writer = EncoderStringWriter;
    let result = writer.flush();
    assert_eq!(result, Ok(()));
}

