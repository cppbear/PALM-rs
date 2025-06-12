// Answer 0

#[test]
fn test_indent_with_zero_iterations() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let result = indent(&mut writer, 0, b"sample data");
}

#[test]
fn test_indent_with_one_iteration_and_write_failure() {
    struct MockFailWriter;

    impl io::Write for MockFailWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockFailWriter;
    let result = indent(&mut writer, 1, b"sample data");
}

#[test]
fn test_indent_with_two_iterations_and_write_failure() {
    struct MockFailWriter;

    impl io::Write for MockFailWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockFailWriter;
    let result = indent(&mut writer, 2, b"sample data");
}

