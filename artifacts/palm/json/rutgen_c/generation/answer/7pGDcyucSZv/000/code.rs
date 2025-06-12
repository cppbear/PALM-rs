// Answer 0

#[test]
fn test_end_object_value_without_writer() {
    struct TestFormatter {
        has_value: bool,
    }

    impl Formatter for TestFormatter {
        fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
            where W: ?Sized + io::Write,
        {
            self.has_value = true;
            Ok(())
        }
    }

    let mut formatter = TestFormatter { has_value: false };
    let result: io::Result<()> = formatter.end_object_value(&mut ());

    assert!(result.is_ok());
    assert!(formatter.has_value);
}

#[test]
fn test_end_object_value_with_writer() {
    struct TestFormatter {
        has_value: bool,
    }

    impl Formatter for TestFormatter {
        fn end_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
            where W: ?Sized + io::Write,
        {
            self.has_value = true;
            writer.write_all(b"end_object_value called")?;
            Ok(())
        }
    }

    let mut formatter = TestFormatter { has_value: false };
    let mut output = Vec::new();
    let result: io::Result<()> = formatter.end_object_value(&mut output);

    assert!(result.is_ok());
    assert!(formatter.has_value);
    assert_eq!(output, b"end_object_value called");
}

