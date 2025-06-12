// Answer 0

#[test]
fn test_new_pretty_formatter() {
    struct MockWriter;
    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let formatter = PrettyFormatter::new();
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, b"  ");
}

#[test]
fn test_pretty_formatter_with_different_indent() {
    let formatter = PrettyFormatter::with_indent(b"\t");
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, b"\t");
}

