// Answer 0

#[test]
fn test_write_literal_class_byte_ascii() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.write_literal_class_byte(b'a').unwrap();
    assert_eq!(writer.output, "a");
}

#[test]
fn test_write_literal_class_byte_control_character() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.write_literal_class_byte(0x01).unwrap();
    assert_eq!(writer.output, "\\x01");
}

#[test]
fn test_write_literal_class_byte_whitespace() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let mut writer_instance = Writer {
        printer: &mut Printer { _priv: () },
        wtr: &mut writer,
    };

    writer_instance.write_literal_class_byte(b' ').unwrap();
    assert_eq!(writer.output, "\\x20");
}

