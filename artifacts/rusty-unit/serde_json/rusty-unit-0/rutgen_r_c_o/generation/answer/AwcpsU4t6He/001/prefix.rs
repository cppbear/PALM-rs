// Answer 0

#[test]
#[should_panic]
fn test_begin_array_value_with_invalid_writer() {
    struct InvalidWriter;

    impl io::Write for InvalidWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"    ",
    };
    
    let mut writer = InvalidWriter;
    formatter.begin_array_value(&mut writer, true).unwrap();
}

