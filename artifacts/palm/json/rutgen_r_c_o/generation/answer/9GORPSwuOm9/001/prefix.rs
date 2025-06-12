// Answer 0

#[test]
fn test_begin_object_key_returns_err_on_writer_error_when_first_true() {
    struct ErroneousWriter;
    
    impl io::Write for ErroneousWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "forced error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = ErroneousWriter;
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"    ",
    };

    let result = formatter.begin_object_key(&mut writer, true);
}

