// Answer 0

#[test]
fn test_write_literal_byte_valid() {
    use std::fmt;

    struct FakeWriter {
        output: String,
    }

    impl fmt::Write for FakeWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut FakeWriter,
    }

    impl<'a> TestStruct<'a> {
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_char(c)
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    let mut writer = FakeWriter { output: String::new() };
    let mut test_struct = TestStruct { wtr: &mut writer };

    // Test valid byte (non-control, non-whitespace, ASCII)
    test_struct.write_literal_byte(65).unwrap(); // 'A'
    assert_eq!(writer.output, "A");
}

#[test]
fn test_write_literal_byte_control_character() {
    use std::fmt;

    struct FakeWriter {
        output: String,
    }

    impl fmt::Write for FakeWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut FakeWriter,
    }

    impl<'a> TestStruct<'a> {
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_char(c)
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    let mut writer = FakeWriter { output: String::new() };
    let mut test_struct = TestStruct { wtr: &mut writer };

    // Test control character (0x00)
    test_struct.write_literal_byte(0x00).unwrap();
    assert_eq!(writer.output, "(?-u:\\x00)");
}

#[test]
fn test_write_literal_byte_whitespace_character() {
    use std::fmt;

    struct FakeWriter {
        output: String,
    }

    impl fmt::Write for FakeWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut FakeWriter,
    }

    impl<'a> TestStruct<'a> {
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_char(c)
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    let mut writer = FakeWriter { output: String::new() };
    let mut test_struct = TestStruct { wtr: &mut writer };

    // Test whitespace character (space)
    test_struct.write_literal_byte(b' ').unwrap();
    assert_eq!(writer.output, "(?-u:\\x20)");
}

#[test]
fn test_write_literal_byte_non_ascii() {
    use std::fmt;

    struct FakeWriter {
        output: String,
    }

    impl fmt::Write for FakeWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
        
        fn write_char(&mut self, c: char) -> fmt::Result {
            self.output.push(c);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut FakeWriter,
    }

    impl<'a> TestStruct<'a> {
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_char(c)
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    let mut writer = FakeWriter { output: String::new() };
    let mut test_struct = TestStruct { wtr: &mut writer };

    // Test non-ASCII character (0xFF)
    test_struct.write_literal_byte(0xFF).unwrap();
    assert_eq!(writer.output, "(?-u:\\xFF)");
}

