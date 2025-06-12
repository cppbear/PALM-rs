// Answer 0

#[test]
fn test_end_object_success() {
    struct MockWriter {
        buffer: Vec<u8>,
    }
    
    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: true,
        indent: b"  ",
    };

    let _ = formatter.end_object(&mut writer);
}

#[test]
#[should_panic]
fn test_end_object_fail_on_write_newline() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "Write error"))
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

    let _ = formatter.end_object(&mut writer);
}

#[test]
#[should_panic]
fn test_end_object_fail_on_indent() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };

    // Adjusting the indent method to produce an error
    fn indent<W>(_wr: &mut W, _n: usize, _s: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Err(io::Error::new(io::ErrorKind::Other, "Indent error"))
    }

    let _ = formatter.end_object(&mut writer);
}

