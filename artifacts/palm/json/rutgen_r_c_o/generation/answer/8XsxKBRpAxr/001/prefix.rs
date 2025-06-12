// Answer 0

#[test]
#[should_panic]
fn test_end_array_with_error_on_write() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };

    let _ = formatter.end_array(&mut writer);
}

