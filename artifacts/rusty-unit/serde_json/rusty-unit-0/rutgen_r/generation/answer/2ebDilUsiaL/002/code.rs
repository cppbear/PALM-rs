// Answer 0

fn test_indent_success() -> std::io::Result<()> {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: vec![] };
    let n = 3;
    let s = b"test";
    
    let result = indent(&mut writer, n, s);
    
    assert!(result.is_ok());
    assert_eq!(writer.output, b"testtesttest");
    Ok(())
}

#[test]
fn test_indent_zero_iterations() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: vec![] };
    let n = 0;
    let s = b"test";
    
    let result = indent(&mut writer, n, s);

    assert!(result.is_ok());
    assert!(writer.output.is_empty());
}

#[test]
#[should_panic]
fn test_indent_panic_on_write_failure() {
    struct PanickingWriter;

    impl std::io::Write for PanickingWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = PanickingWriter;
    let n = 2;
    let s = b"test";

    let _ = indent(&mut writer, n, s);
}

