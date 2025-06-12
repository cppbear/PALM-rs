// Answer 0

#[test]
fn test_finish() {
    struct DummyWriter;
    impl fmt::Write for DummyWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = DummyWriter;
    let visitor = Writer {
        printer: &mut Printer { _priv: () },
        wtr: writer,
    };
    let result = visitor.finish();
    assert!(result.is_ok());
}

