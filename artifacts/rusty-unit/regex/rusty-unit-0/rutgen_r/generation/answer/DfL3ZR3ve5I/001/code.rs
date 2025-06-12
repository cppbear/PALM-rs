// Answer 0

#[test]
fn test_fmt_with_whitespace_start_and_end() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    impl ClassUnicodeRange {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let start = if !self.start.is_whitespace() && !self.start.is_control() {
                self.start.to_string()
            } else {
                format!("0x{:X}", self.start as u32)
            };
            let end = if !self.end.is_whitespace() && !self.end.is_control() {
                self.end.to_string()
            } else {
                format!("0x{:X}", self.end as u32)
            };
            f.debug_struct("ClassUnicodeRange")
                .field("start", &start)
                .field("end", &end)
                .finish()
        }
    }

    let range = ClassUnicodeRange {
        start: ' ',
        end: '\n',
    };

    let mut output = std::fmt::Formatter::new();
    range.fmt(&mut output).unwrap();
}

#[test]
fn test_fmt_with_control_characters_start_and_end() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    impl ClassUnicodeRange {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let start = if !self.start.is_whitespace() && !self.start.is_control() {
                self.start.to_string()
            } else {
                format!("0x{:X}", self.start as u32)
            };
            let end = if !self.end.is_whitespace() && !self.end.is_control() {
                self.end.to_string()
            } else {
                format!("0x{:X}", self.end as u32)
            };
            f.debug_struct("ClassUnicodeRange")
                .field("start", &start)
                .field("end", &end)
                .finish()
        }
    }

    let range = ClassUnicodeRange {
        start: '\0',
        end: '\x1F',
    };

    let mut output = std::fmt::Formatter::new();
    range.fmt(&mut output).unwrap();
}

