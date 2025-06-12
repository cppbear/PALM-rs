// Answer 0

#[test]
fn test_write_literal_byte_with_control_character() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut DummyWriter,
    }

    impl TestStruct<'_> {
        fn write_literal_byte(&mut self, b: u8) -> std::fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_str(&c.to_string())
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    let mut writer = DummyWriter::new();
    let mut test_struct = TestStruct { wtr: &mut writer };

    // Testing boundary case for c == 0x7F
    let result = test_struct.write_literal_byte(0x7F);

    assert_eq!(result.is_err(), true);
    assert_eq!(writer.output, "(?-u:\\x7F)");
}

#[test]
fn test_write_literal_byte_with_whitespace() {
    struct DummyWriter {
        output: String,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter {
                output: String::new(),
            }
        }
    }

    impl std::fmt::Write for DummyWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut DummyWriter,
    }

    impl TestStruct<'_> {
        fn write_literal_byte(&mut self, b: u8) -> std::fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_str(&c.to_string())
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    let mut writer = DummyWriter::new();
    let mut test_struct = TestStruct { wtr: &mut writer };

    // Testing whitespace character (space)
    let result = test_struct.write_literal_byte(b' ');

    assert_eq!(result.is_err(), true);
    assert_eq!(writer.output, "(?-u:\\x20)");
}

