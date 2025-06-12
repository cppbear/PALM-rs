// Answer 0

#[test]
fn test_indent_zero_iterations() {
    use std::io;

    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { buffer: Vec::new() };
    let n = 0;
    let s: &[u8] = b"    "; // Four spaces.

    let result = indent(&mut writer, n, s);
    assert!(result.is_ok());
    assert!(writer.buffer.is_empty());
}

