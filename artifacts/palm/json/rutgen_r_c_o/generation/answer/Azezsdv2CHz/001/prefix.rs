// Answer 0

#[test]
fn test_end_object_with_writer_error() {
    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "fail"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: true,
        indent: b"  ",
    };
    
    let mut writer = FailingWriter;
    
    let _ = formatter.end_object(&mut writer);
}

#[test]
fn test_end_object_with_higher_indent() {
    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "fail"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 3,
        has_value: true,
        indent: b"    ",
    };

    let mut writer = FailingWriter;

    let _ = formatter.end_object(&mut writer);
}

#[test]
fn test_end_object_with_only_one_indent() {
    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "fail"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };

    let mut writer = FailingWriter;

    let _ = formatter.end_object(&mut writer);
}

