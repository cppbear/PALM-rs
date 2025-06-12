// Answer 0

#[test]
fn test_collect_str_success() {
    use std::fmt::{self, Write};
    use std::io;

    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        in_string: bool,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { in_string: false }
        }

        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<(), io::Error> {
            self.in_string = true;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<(), io::Error> {
            self.in_string = false;
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter::new();

    let result = collect_str(&mut writer, &mut formatter, "Hello, world!");

    assert!(result.is_ok());
    assert_eq!(writer.output, "Hello, world!");
}

#[test]
#[should_panic]
fn test_collect_str_writer_error() {
    use std::fmt::{self, Write};
    use std::io;

    struct FaultyWriter;

    impl io::Write for FaultyWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter
        }

        fn begin_string(&self, _writer: &mut FaultyWriter) -> Result<(), io::Error> {
            Ok(())
        }

        fn end_string(&self, _writer: &mut FaultyWriter) -> Result<(), io::Error> {
            Ok(())
        }
    }

    let mut writer = FaultyWriter;
    let formatter = MockFormatter::new();

    let _ = collect_str(&mut writer, &formatter, "Hello, world!");
}

