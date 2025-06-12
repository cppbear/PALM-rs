// Answer 0

#[derive(Debug)]
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

#[test]
fn test_class_unicode_range_fmt_non_whitespace_control() {
    let range = ClassUnicodeRange {
        start: 'a',
        end: 'z',
    };
    let mut buf = Vec::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut buf);
        range.fmt(formatter).unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("start: \"a\""));
    assert!(output.contains("end: \"z\""));
}

#[test]
fn test_class_unicode_range_fmt_boundary_values() {
    let range = ClassUnicodeRange {
        start: '0',
        end: '9',
    };
    let mut buf = Vec::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut buf);
        range.fmt(formatter).unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("start: \"0\""));
    assert!(output.contains("end: \"9\""));
}

#[test]
fn test_class_unicode_range_fmt_extreme_values() {
    let range = ClassUnicodeRange {
        start: '\u{007F}', // DEL (control character)
        end: '\u{FFFF}',   // Non-control character
    };
    let mut buf = Vec::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut buf);
        range.fmt(formatter).unwrap();
    }
    let output = String::from_utf8(buf).unwrap();
    assert!(output.contains("0x7F"));
    assert!(output.contains("end: \"\\u{FFFF}\""));
}

