// Answer 0

#[test]
fn test_write_literal_char_non_meta() {
    use std::fmt::Write;

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

    let mut writer = TestWriter::new();
    let c = 'a'; // Non-meta character

    let result = writer.write_char(c);
    assert!(result.is_ok());
    assert_eq!(writer.output, "a");
}

#[test]
fn test_write_literal_char_meta() {
    use std::fmt::Write;

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_char(&mut self, c: char) -> fmt::Result {
            if is_meta_character(c) {
                self.output.push_str("\\")?;
            }
            self.output.push(c)?;
            Ok(())
        }
    }

    fn is_meta_character(c: char) -> bool {
        match c {
            '.' | '^' | '$' | '*' | '+' | '?' | '|' | '(' | ')' | '[' | ']' | '{' | '}' => true,
            _ => false,
        }
    }

    let mut writer = TestWriter::new();
    let c = '.'; // Meta character

    let result = writer.write_char(c);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\\.");
}

