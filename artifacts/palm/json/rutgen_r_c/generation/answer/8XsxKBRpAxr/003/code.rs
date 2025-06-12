// Answer 0

#[test]
fn test_end_array_with_has_value() {
    struct TestWriter {
        output: Vec<u8>,
        error: Option<Error>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if let Some(ref err) = self.error {
                return Err(err.clone());
            }
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut test_writer = TestWriter { output: Vec::new(), error: None };
    let indent: &[u8] = b"    "; // 4 spaces
    let mut formatter = PrettyFormatter { current_indent: 1, has_value: true, indent };

    let result = formatter.end_array(&mut test_writer);
    
    assert!(result.is_ok());
    assert_eq!(test_writer.output, b"\n" /* newline */);
    assert_eq!(formatter.current_indent, 0);
} 

#[test]
fn test_end_array_without_has_value() {
    struct TestWriter {
        output: Vec<u8>,
        error: Option<Error>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if let Some(ref err) = self.error {
                return Err(err.clone());
            }
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut test_writer = TestWriter { output: Vec::new(), error: None };
    let indent: &[u8] = b"    "; // 4 spaces
    let mut formatter = PrettyFormatter { current_indent: 1, has_value: false, indent };

    let result = formatter.end_array(&mut test_writer);
    
    assert!(result.is_ok());
    assert_eq!(test_writer.output, b"]");
    assert_eq!(formatter.current_indent, 0);
}

#[test]
fn test_end_array_with_writer_error() {
    struct TestWriter {
        error: Option<Error>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(self.error.clone().unwrap())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let error = Error::new(ErrorCode::IoError);
    let mut test_writer = TestWriter { error: Some(error.clone()) };
    let indent: &[u8] = b"    "; // 4 spaces
    let mut formatter = PrettyFormatter { current_indent: 1, has_value: true, indent };

    let result = formatter.end_array(&mut test_writer);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), error);
    assert_eq!(formatter.current_indent, 0);
} 

#[test]
fn test_end_array_with_indent_error() {
    struct TestWriter {
        output: Vec<u8>,
        error: Option<Error>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if let Some(ref err) = self.error {
                return Err(err.clone());
            }
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn faulty_indent<W>(_wr: &mut W, _n: usize, _s: &[u8]) -> io::Result<()> 
    where W: ?Sized + io::Write {
        Err(Error::new(ErrorCode::IndentError))
    }

    let mut test_writer = TestWriter { output: Vec::new(), error: None };
    let indent: &[u8] = b"    "; // 4 spaces
    let mut formatter = PrettyFormatter { current_indent: 1, has_value: true, indent };

    let result = formatter.end_array(&mut test_writer);
    
    assert!(result.is_err());
    assert_eq!(formatter.current_indent, 0);
}

