// Answer 0

#[test]
fn test_indent_success() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let result = indent(&mut writer, 3, b"test");
    assert!(result.is_ok());
    assert_eq!(writer.data, b"testtesttest");
}

#[test]
fn test_indent_zero_runs() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let result = indent(&mut writer, 0, b"test");
    assert!(result.is_ok());
    assert_eq!(writer.data.len(), 0);
}

