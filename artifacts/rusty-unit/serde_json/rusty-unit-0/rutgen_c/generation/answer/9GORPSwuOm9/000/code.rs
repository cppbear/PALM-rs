// Answer 0

#[test]
fn test_begin_object_key_first() {
    struct TestWriter(Vec<u8>);

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent: b"  ",
    };

    formatter.begin_object_key(&mut writer, true).unwrap();

    assert_eq!(writer.0, b"\n");
}

#[test]
fn test_begin_object_key_not_first() {
    struct TestWriter(Vec<u8>);

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter(Vec::new());
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent: b"  ",
    };

    formatter.begin_object_key(&mut writer, false).unwrap();

    assert_eq!(writer.0, b",\n");
}

