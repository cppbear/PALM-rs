// Answer 0

fn test_write_literal_byte() {
    use std::fmt::{self, Write};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct<W: Write> {
        wtr: W,
    }

    impl<W: Write> TestStruct<W> {
        fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
            let c = b as char;
            if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
                self.wtr.write_char(c)
            } else {
                write!(self.wtr, "(?-u:\\x{:02X})", b)
            }
        }
    }

    // Test case for boundary condition: c == 0x7F as char (which is a control character)
    let mut writer = TestWriter::new();
    {
        let mut test_struct = TestStruct { wtr: &mut writer };
        let result = test_struct.write_literal_byte(0x7F);
        assert!(result.is_ok());
        assert_eq!(writer.output, "(?-u:\\x7F)");
    }

    // Test case for a control character: c is control (0x00)
    writer.output.clear();
    {
        let mut test_struct = TestStruct { wtr: &mut writer };
        let result = test_struct.write_literal_byte(0x00);
        assert!(result.is_ok());
        assert_eq!(writer.output, "(?-u:\\x00)");
    }

    // Test case for a valid printable ASCII character (0x41 for 'A')
    writer.output.clear();
    {
        let mut test_struct = TestStruct { wtr: &mut writer };
        let result = test_struct.write_literal_byte(0x41);
        assert!(result.is_ok());
        assert_eq!(writer.output, "A");
    }
}

